use std::{
    fmt, panic,
    sync::{Arc, Mutex, TryLockError},
    thread,
};

use crossbeam_channel::{unbounded, Receiver, Sender};
use gdnative::prelude::*;
use image::RgbaImage;
use itertools::{Either, Itertools};
use log::*;
use opencv::{
    core::{Mat, Size2i, Vec2f, CV_32FC2},
    optflow::{self, InterpolationType, RLOFOpticalFlowParameter},
    prelude::*,
};

use crate::{
    datatypes::{FlowAlg, FlowAlg::*, ImgParams},
    frame::{flow_mat_to_frame, Frame, Frames},
    utility::*,
};

type Base = Node;
//Base refers to the type ImageProcessor inherits from. In this case it's Node (because #[inherit(Node)])

type MaybeError = Option<String>;

//#[derive(Debug)]
#[non_exhaustive]
pub enum ImageProcessorMessage {
    UpdatedInputImage(Frames),
    UpdatedImgParams(ImgParams),
}

impl fmt::Debug for ImageProcessorMessage {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ImageProcessorMessage::UpdatedInputImage(_) => write!(formatter, "UpdatedInputImage"),
            ImageProcessorMessage::UpdatedImgParams(_) => write!(formatter, "UpdatedImgParams"),
        }
    }
}

#[derive(Default)]
struct ImageProcessorInner {
    input_frames: Frames,
    img_params: ImgParams,
}

#[derive(NativeClass)]
#[inherit(Base)]
#[register_with(Self::register_signals)]
pub struct ImageProcessor {
    output_image_channel: Arc<(Sender<Frames>, Receiver<Frames>)>,
    error_channel: Arc<(Sender<MaybeError>, Receiver<MaybeError>)>,
    update_channel: Arc<(
        Sender<ImageProcessorMessage>,
        Receiver<ImageProcessorMessage>,
    )>,
    progress_channel: Arc<(Sender<f64>, Receiver<f64>)>,
    inner: Arc<Mutex<ImageProcessorInner>>,
}

impl ImageProcessor {
    fn new(_owner: &Node) -> Self {
        ImageProcessor {
            output_image_channel: Arc::new(unbounded()),
            error_channel: Arc::new(unbounded()),
            update_channel: Arc::new(unbounded()),
            progress_channel: Arc::new(unbounded()),
            inner: Default::default(),
        }
    }

    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "image_processed",
            args: &[SignalArgument {
                name: "frames",
                default: Variant::from_array(&VariantArray::new_shared()),
                export_info: ExportInfo::new(VariantType::VariantArray),
                usage: PropertyUsage::DEFAULT,
            }],
        });
        builder.add_signal(Signal {
            name: "made_progress",
            args: &[SignalArgument {
                name: "progress",
                default: Variant::from_f64(0.0),
                export_info: ExportInfo::new(VariantType::F64),
                usage: PropertyUsage::DEFAULT,
            }],
        });
        builder.add_signal(Signal {
            name: "error_occured",
            args: &[SignalArgument {
                name: "error",
                default: Variant::from_str("Dummy error"),
                export_info: ExportInfo::new(VariantType::GodotString),
                usage: PropertyUsage::DEFAULT,
            }],
        });
    }

    fn get_image_processor_closure(&self) -> Box<dyn Send + Fn()> {
        let output_img_sender = Arc::clone(&self.output_image_channel).0.clone();
        let error_sender = Arc::clone(&self.error_channel).0.clone();
        let progress_sender = Arc::clone(&self.progress_channel).0.clone();
        let inner = Arc::clone(&self.inner);

        let update_channel = Arc::clone(&self.update_channel);
        let update_receiver = update_channel.1.clone();
        let has_pending_messages = move || !update_receiver.is_empty();

        Box::new(move || {
            let update_receiver = update_channel.1.clone();

            loop {
                let (new_params, new_frames) = wait_and_retain_latest_messages(&update_receiver);

                let lock = inner.lock();
                let mut inner = match lock {
                    Ok(inner) => inner,
                    Err(err) => {
                        warn!("WARNING: mutex poisoned: {:?}", err);
                        err.into_inner()
                    }
                };

                if let Some(params) = new_params {
                    inner.img_params = params;
                }

                if let Some(img) = new_frames {
                    inner.input_frames = img;
                }

                let thread_result = panic::catch_unwind(|| {
                    let mut input_frames: Vec<&Frame> = inner.input_frames.iter().collect();
                    //NOTE: input_frames contains references to inner.input_frames

                    let mut frame_count = input_frames.len();
                    if frame_count == 0 {
                        //Haven't received any frames yet, retrying......
                        return;
                    }

                    let ImgParams {
                        mut inbetweens,
                        loop_seamlessly,
                        flow_multiplier: total_flow_multiplier,
                        optflow_alg,
                        show_motion_vectors,
                    } = inner.img_params.clone();

                    if show_motion_vectors {
                        inbetweens = 1;
                    }

                    let mut flow_multipliers = vec![];
                    for i in 0..(inbetweens + 1) {
                        flow_multipliers.push(i as f32 / (inbetweens + 1) as f32);
                    }

                    //Copy the first frame reference to the end of the vector so it loops
                    if loop_seamlessly {
                        input_frames.push(input_frames.first().unwrap());
                        frame_count = input_frames.len();
                    } else {
                        //Else duplicate the last frame reference
                        input_frames.push(input_frames.last().unwrap());
                        frame_count = input_frames.len();
                    }

                    let mut output_frames: Frames = vec![];
                    progress_sender.send(f64::EPSILON).unwrap();
                    error_sender.send(None).unwrap(); //Clear previous error

                    let mut i = 0;
                    for (frame_a, frame_b) in input_frames.windows(2).map(array_to_pair) {
                        if has_pending_messages() {
                            //Cancelling current calculation
                            return;
                        }

                        //Note that this throws away alpha information
                        let mat_a_bgr = Mat::from(*frame_a);
                        let mat_b_bgr = Mat::from(*frame_b);

                        assert!(mat_a_bgr.depth() == opencv::core::CV_8U);
                        assert!(mat_b_bgr.depth() == opencv::core::CV_8U);
                        assert!(mat_a_bgr.channels() == 3);
                        assert!(mat_b_bgr.channels() == 3);
                        assert!(
                            mat_a_bgr.size().unwrap() == mat_b_bgr.size().unwrap(),
                            "Frames don't have the same size: {:?} != {:?}",
                            mat_a_bgr.size().unwrap(),
                            mat_b_bgr.size().unwrap()
                        );
                        let mut flow =
                            unsafe { Mat::new_size(mat_a_bgr.size().unwrap(), CV_32FC2).unwrap() };

                        match Self::do_optical_flow(&mat_a_bgr, &mat_b_bgr, &mut flow, &optflow_alg)
                        {
                            Ok(()) => {}
                            Err(err) => {
                                warn!("Optical flow failed: {:?}", err);
                                error_sender.send(Some(err.to_string())).unwrap();
                                return;
                            }
                        }

                        if show_motion_vectors {
                            output_frames.push(flow_mat_to_frame(&flow));
                            i += 1;
                            progress_sender
                                .send(i as f64 / (frame_count - 1) as f64)
                                .unwrap();
                        } else {
                            for flow_multiplier in &flow_multipliers {
                                let flow_multiplier = flow_multiplier * total_flow_multiplier;

                                i += 1;
                                progress_sender
                                    .send(
                                        i as f64
                                            / ((frame_count - 1) * flow_multipliers.len()) as f64,
                                    )
                                    .unwrap();

                                //Skip flow calc if no flow
                                let output_buffer: Frame = if abs_diff_eq!(flow_multiplier, 0.0) {
                                    (*frame_a).clone()
                                } else {
                                    Self::apply_flow_to(frame_a, &flow, flow_multiplier)
                                };
                                output_frames.push(output_buffer);
                            }
                        }
                    }
                    progress_sender.send(1.0).unwrap(); //done

                    output_img_sender.send(output_frames).unwrap();
                });

                if let Err(err) = thread_result {
                    let mut err_str =
                        format!("Unknown error '{:?}' with type {:?}", err, err.type_id());

                    if let Some(dynamic_string) = err.downcast_ref::<String>() {
                        err_str = dynamic_string.clone();
                    } else if let Some(static_string) = err.downcast_ref::<&str>() {
                        err_str = static_string.to_string();
                    }

                    warn!("ImageProcessor thread had a panic attack: {:?}", err_str);
                    error_sender.send(Some(err_str)).unwrap();
                }
            }
        })
    }
}

#[methods]
impl ImageProcessor {
    #[export]
    fn _on_ui_img_params_changed(&mut self, _owner: TRef<'_, Base>, img_params: ImgParams) {
        self.update_channel
            .0
            .send(ImageProcessorMessage::UpdatedImgParams(img_params))
            .unwrap();
    }

    #[export]
    fn _on_imageholder_image_loaded(&mut self, _owner: TRef<'_, Base>, frames: Frames) {
        self.update_channel
            .0
            .send(ImageProcessorMessage::UpdatedInputImage(frames))
            .unwrap();
    }

    #[export]
    fn _ready(&mut self, _owner: TRef<'_, Base>) {
        self.progress_channel.0.send(0.0).unwrap();

        thread::Builder::new()
            .name("image_processor_thread".to_string())
            .spawn(self.get_image_processor_closure())
            .expect("failed to start thread");
    }

    #[export]
    fn _process(&self, owner: &Base, _delta: f32) {
        for progress in self.progress_channel.1.try_iter() {
            owner.emit_signal("made_progress", &[progress.to_variant()]);
        }

        if let Ok(frames) = self.output_image_channel.1.try_recv() {
            owner.emit_signal("image_processed", &[frames.to_variant()]);
        }

        if let Ok(maybe_err) = self.error_channel.1.try_recv() {
            owner.emit_signal(
                "error_occured",
                &[maybe_err.map_or(Variant::new(), |str| str.to_variant())],
            );
        }
    }

    #[export]
    fn is_busy(&self, _owner: &Base) -> bool {
        let is_busy = match self.inner.try_lock() {
            Err(TryLockError::WouldBlock) => true,
            Err(TryLockError::Poisoned(err)) => {
                warn!("WARNING: mutex poisoned: {:?}", err);
                false
            }
            _ => false,
        };

        is_busy
    }

    fn apply_flow_to(frame: &Frame, flow: &Mat, flow_multiplier: f32) -> Frame {
        let w = frame.width();
        let h = frame.height();

        let inner = RgbaImage::from_fn(w, h, |x, y| {
            let mut flow: Vec2f = *flow.at_2d(y as i32, x as i32).unwrap();

            if !flow[0].is_finite() || !flow[1].is_finite() {
                *flow = [0.0, 0.0];
            }

            //TODO don't round if pixelmode = false, instead use bilinear filter
            let new_x: i32 =
                ((x as f32 - flow[0] * flow_multiplier).round() as i32).clamp(0, (w - 1) as i32);
            let new_y: i32 =
                ((y as f32 - flow[1] * flow_multiplier).round() as i32).clamp(0, (h - 1) as i32);

            frame[(new_x as u32, new_y as u32)]
        });

        Frame(inner)
    }

    fn do_optical_flow(
        mat_a: &Mat,
        mat_b: &Mat,
        flow: &mut Mat,
        optflow_alg: &FlowAlg,
    ) -> Result<(), opencv::Error> {
        match *optflow_alg {
            SimpleFlow {
                layers,
                averaging_block_size,
                max_flow,
            } => optflow::calc_optical_flow_sf(
                //see simple_flow_demo.cpp https://github.com/npinto/opencv/blob/master/samples/cpp/simpleflow_demo.cpp#L90
                mat_a,
                mat_b,
                flow,
                layers as i32,
                averaging_block_size as i32,
                max_flow as i32,
            ),
            DenseRLOF {
                forward_backward_threshold,
                grid_step_x,
                grid_step_y,
                use_post_proc,
                use_variational_refinement,
            } => {
                let rlof_param = RLOFOpticalFlowParameter::create().unwrap();
                let grid_step = Size2i::new(grid_step_x, grid_step_y);
                let interp_type = InterpolationType::INTERP_EPIC;
                optflow::calc_optical_flow_dense_rlof(
                    mat_a,
                    mat_b,
                    flow,
                    rlof_param,
                    forward_backward_threshold,
                    grid_step,
                    interp_type,
                    128,
                    0.05,
                    100.0,
                    15,
                    100,
                    use_post_proc,
                    500.0,
                    1.5,
                    use_variational_refinement,
                )
            }
            ref x => {
                panic!("Unknown optical flow algorithm {:?}", x);
            }
        }
    }
}

fn wait_and_retain_latest_messages(
    update_receiver: &Receiver<ImageProcessorMessage>,
) -> (Option<ImgParams>, Option<Frames>) {
    //This method does 3 things:
    //1. Wait for a message to arrive (blocking)
    //2. Collect all pending messages in a list
    //3. Retain only the latest message, per message type

    let peeked = update_receiver.iter().next().unwrap();
    let mut msgs: Vec<ImageProcessorMessage> = update_receiver.try_iter().collect();
    msgs.insert(0, peeked);

    let (mut msgs_params, mut msgs_frames): (Vec<_>, Vec<_>) =
        msgs.into_iter().partition_map(|x| match x {
            ImageProcessorMessage::UpdatedImgParams(params) => Either::Left(params),
            ImageProcessorMessage::UpdatedInputImage(frames) => Either::Right(frames),
        });

    //TODO use hashmap instead?

    (msgs_params.pop(), msgs_frames.pop())
}

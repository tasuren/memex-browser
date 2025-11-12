use std::time::Duration;

use futures::FutureExt;

pub struct PumpDelayMs(pub u64);
pub type PumpTx = async_channel::Sender<PumpDelayMs>;
pub type PumpRx = async_channel::Receiver<PumpDelayMs>;

pub struct EventLoopHandle {
    pub pump_rx: PumpRx,
}

impl EventLoopHandle {
    pub(crate) fn new() -> (Self, PumpTx) {
        let (pump_tx, pump_rx) = async_channel::unbounded();

        (Self { pump_rx }, pump_tx)
    }

    const FRAME_RATE_DELAY: Duration = Duration::from_millis(1000 / 60);

    pub async fn start(&mut self, run_on_main_thread: impl Fn(fn())) {
        let mut delay = Self::FRAME_RATE_DELAY;

        loop {
            futures::select! {
                delay_ms = self.pump_rx.recv().fuse() => {
                    if let Ok(delay_ms) = delay_ms {
                        if delay_ms.0 == 0 {
                            run_on_main_thread(cef::do_message_loop_work);
                        } else {
                            delay = Duration::from_millis(delay_ms.0);
                        }
                    } else {
                        break;
                    }
                }
                _ = async_io::Timer::after(delay).fuse() => {
                    run_on_main_thread(cef::do_message_loop_work);
                    delay = Self::FRAME_RATE_DELAY;
                }
            }
        }
    }
}

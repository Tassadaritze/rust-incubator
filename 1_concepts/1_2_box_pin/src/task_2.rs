use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Instant;

struct MeasurableFuture<Fut> {
    inner_future: Fut,
    started_at: Option<Instant>,
}

impl<Fut: Future> Future for MeasurableFuture<Fut> {
    type Output = Fut::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let start_time = self.started_at;
        let pinned_inner = unsafe {
            // SAFETY: this is OK because `self.inner_future` is only accessed while pinned.
            self.map_unchecked_mut(|s| &mut s.inner_future)
        };

        let poll = pinned_inner.poll(cx);
        if matches!(poll, Poll::Ready(_)) {
            if let Some(start_time) = start_time {
                println!("took {} ns", (Instant::now() - start_time).as_nanos());
            }
        }

        poll
    }
}

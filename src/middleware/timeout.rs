// pub struct Timeout<S> {
//     service: S,
//     timeout: Duration,
// }

// impl<S> Service for Timeout<S>
// where
//     S: Service,
// {
//     type Request = S::Request;
//     type Response = S::Response;
//     type Error = TimeoutError<S::Error>;
//     type Future = TimeoutServiceResponse<S>;

//     fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         ready!(self.service.poll_ready(cx)).map_err(TimeoutError::Service)
//     }

//     fn call(&mut self, req: S::Request) -> Self::Future {
//         TimeoutServiceResponse {
//             fut: self.service.call(req),
//             sleep: Delay::new(clock::now() + self.timeout),
//         }
//     }
// }
use pavex::blueprint::Blueprint;
use pavex::connection::ConnectionInfo;
use pavex::f;
use pavex::request::{
    body::RawIncomingBody,
    path::{MatchedPathPattern, RawPathParams},
    RequestHead,
};
use pavex::router::AllowedMethods;

pub fn handler(
    _info: &ConnectionInfo,
    _head: &RequestHead,
    _body: &RawIncomingBody,
    _methods: &AllowedMethods,
    _pattern: &MatchedPathPattern,
    _params: &RawPathParams,
) -> pavex::response::Response {
    todo!()
}

pub fn blueprint() -> Blueprint {
    let mut bp = Blueprint::new();
    bp.prefix("/nested").nest({
        let mut bp = Blueprint::new();
        bp.fallback(f!(crate::handler));
        bp
    });
    bp.fallback(f!(crate::handler));
    bp
}

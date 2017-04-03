extern crate iron;
extern crate urlencoded;

use std::error::Error;
use self::iron::{Request, IronResult, Response, IronError, status};
use self::iron::prelude::*;
use self::urlencoded::{UrlEncodedBody, UrlDecodingError};

pub fn get_formdata(req: &mut Request, form_id: &str) -> IronResult<String> {
    let formdata = req.get_ref::<UrlEncodedBody>();
    let formdata = to_ironresult(formdata)?;

    let data = formdata.get(form_id)
        .ok_or(IronError {
                   error: (Box::new(UrlDecodingError::EmptyQuery)),
                   response: Response::with(status::BadRequest),
               })?;
    Ok(data[0].to_owned())
}

pub fn to_ironresult<T, E>(result: Result<T, E>) -> IronResult<T>
    where E: Send + Error + 'static
{
    result.map_err(|err| {
                       IronError {
                           error: Box::new(err),
                           response: Response::with(status::BadRequest),
                       }
                   })
}

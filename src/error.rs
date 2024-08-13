use std::error::Error;
use std::fmt;

pub type Result<T> = std::result::Result<T, GeckError>;
pub type BoxError = Box<dyn Error + Send + Sync>;

pub struct Inner {
	kind: ErrorKind,
	source: Option<BoxError>,
}

pub struct GeckError {
	details: String, 	
	inner: Box<Inner>,
}

impl GeckError {
    pub fn new<E>(kind: ErrorKind, source: Option<E>, msg: &str) -> Self 
			where E: Into<BoxError> {
				Self {
					inner: Box::new(Inner { kind: kind, source: source.map(Into::into)}),
					details: msg.to_owned(),
				}
    }
}

impl fmt::Debug for GeckError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("rust_geck::GeckError")
			.field("kind", &self.inner.kind)
			.finish()
	}
}

impl fmt::Display for GeckError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.inner.kind {
			ErrorKind::Driver => write!(f, "Driver Error: {}", self.details),
			ErrorKind::Service => write!(f, "Service Error: {}", self.details),
			ErrorKind::Context => write!(f, "Context Error: {}", self.details),
			ErrorKind::Gecko => write!(f, "Gecko Error: {}", self.details),
			ErrorKind::Other => write!(f, "Error: {}", self.details),
		}
	}
}

impl Error for GeckError {
    fn description(&self) -> &str {
        &self.details
    }
		fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
			self.inner.source.as_ref().map(|e| &**e as _)
		}
}

impl From<reqwest::Error> for GeckError {
	fn from(err: reqwest::Error) -> GeckError {
		GeckError::new(ErrorKind::Driver, Some(err), "Driver Failed")
	}
}

#[derive(Debug)]
pub enum ErrorKind {
	Driver,
	Service,
	Context,
	Gecko,
	Other,
}

#[cfg(test)]
mod tests {
    use super::*;
		// TODO write a proper test
		// a test function that returns our error result
		fn raises_my_error(yes: bool) -> std::result::Result<(),GeckError> {
				if yes {
						let source = GeckError::new(ErrorKind::Other, None::<GeckError>, "This");
						Err(GeckError::new(ErrorKind::Other, Some(source) , "This"))
				} else {
						Ok(())
				}
		}
		fn raises_my_error_2(yes: bool) -> std::result::Result<(),GeckError> {
				if yes {
						Err(GeckError::new(ErrorKind::Other, Some("Test") , "This"))
				} else {
						Ok(())
				}
		}
}



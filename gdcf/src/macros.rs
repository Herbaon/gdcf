macro_rules! collect_one {
    ($cache: expr, $variant: ident) => {
        move |response| {
            let mut result = None;

            for obj in response {
                $cache.store_object(&obj).map_err(GdcfError::Cache)?;

                if let GDObject::$variant(level) = obj {
                    result = Some(level)
                }
            }

            result.ok_or(GdcfError::NoContent)
        }
    };
}

macro_rules! collect_many {
    ($request: expr, $cache: expr, $bulk_store: ident, $variant: ident) => {
        move |response| {
            let mut result = Vec::new();

            for obj in response {
                $cache.store_object(&obj).map_err(GdcfError::Cache)?;

                if let GDObject::$variant(level) = obj {
                    result.push(level)
                }
            }

            if !result.is_empty() {
                $cache.$bulk_store(&$request, &result).map_err(GdcfError::Cache)?;

                Ok(result)
            } else {
                Err(GdcfError::NoContent)
            }
        }
    };
}

macro_rules! gdcf {
    ($self: expr, $request: expr, $cache_lookup: ident, $future_closure: expr) => {{
        let cache = $self.cache();

        match cache.$cache_lookup(&$request) {
            Ok(cached) =>
                if cache.is_expired(&cached) {
                    info!("Cache entry for request {} is expired!", $request);

                    GdcfFuture::outdated(cached, $future_closure())
                } else {
                    info!("Cached entry for request {} is up-to-date!", $request);

                    GdcfFuture::up_to_date(cached)
                },

            Err(error) =>
                if error.is_cache_miss() {
                    info!("No cache entry for request {}", $request);

                    GdcfFuture::absent($future_closure())
                } else {
                    GdcfFuture::cache_error(error)
                },
        }
    }};
}

macro_rules! setter {
    ($name: ident, $field: ident, $t: ty) => {
        pub fn $name(mut self, $field: $t) -> Self {
            self.$field = $field;
            self
        }
    };

    ($name: ident, $t: ty) => {
        pub fn $name(mut self, arg0: $t) -> Self {
            self.$name = arg0;
            self
        }
    };

    ($(#[$attr:meta])* $name: ident: $t: ty) => {
        $(#[$attr])*
        pub fn $name(mut self, $name: $t) -> Self {
            self.$name = $name;
            self
        }
    };

    ($(#[$attr:meta])* $field:ident[$name: ident]: $t: ty) => {
        $(#[$attr])*
        pub fn $name(mut self, $field: $t) -> Self {
            self.$field = $field;
            self
        }
    }
}

macro_rules! const_setter {
    ($name: ident, $field: ident, $t: ty) => {
        pub const fn $name(mut self, $field: $t) -> Self {
            self.$field = $field;
            self
        }
    };

    ($name: ident, $t: ty) => {
        pub const fn $name(mut self, arg0: $t) -> Self {
            self.$name = arg0;
            self
        }
    };

    ($(#[$attr:meta])* $name: ident: $t: ty) => {
        $(#[$attr])*
        pub const fn $name(mut self, $name: $t) -> Self {
            self.$name = $name;
            self
        }
    };

    ($(#[$attr:meta])* $field:ident[$name: ident]: $t: ty) => {
        $(#[$attr])*
        pub const fn $name(mut self, $field: $t) -> Self {
            self.$field = $field;
            self
        }
    }
}
/*
macro_rules! from_str {
    ($target: ident) => {
        impl FromStr for $target {
            type Err = ParseIntError;

            fn from_str(s: &str) -> Result<$target, ParseIntError> {
                let value: i32 = s.parse()?;

                Ok($target::from(value))
            }
        }
    };
}
*/

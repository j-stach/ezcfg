
/// Define field names and types for configurable variables.
/// ```
/// ezcfg::cfg!{ 
///     TestConfig ["~/test/path.cfg"] 
///         field1: String,
///         field2: u32
/// }
///
/// let test = TestConfig { 
///     field1: String::new(), 
///     field2: 0u32 
/// };
///
/// assert_eq!(test.field1(), &String::new());
/// assert_eq!(test.field2(), &0u32);
/// ```
#[macro_export]
macro_rules! cfg {
    ($name:ident [$path:expr] $($field:ident: $typ:ident),* $(,)?) => {

        pub(crate) struct $name {
            $(pub(crate) $field: $typ,)*
        }

        impl $name {
            pub(crate) fn new($($field: $typ,)*) -> Self { Self {$( $field, )*}}
            $(pub(crate) fn $field(&self) -> &$typ { &self.$field })*
        }

        impl $crate::Config for $name {

            const PATH: &str = $path;

            fn read() -> Result<Self, $crate::Error> {

                let file = std::fs::read_to_string(Self::PATH)?;
                let lines: Vec<&str> = file.split('\n').collect();

                let mut fields = Vec::with_capacity(lines.len());

                for line in lines {
                    let fv: Vec<&str> = line.split('=').collect();
                    if fv.len() != 2 {
                        return Err($crate::Error::Format(line.to_owned()))
                    }
                    fields.push((fv[0].to_owned(), fv[1].to_owned()))
                }

                Ok(Self {
                    $($field: {
                        let field = stringify!($field);
                        let (_, value): &(String, String) = fields.iter()
                            .find(|fv| fv.0 == field)
                            .ok_or($crate::Error::Missing(field.to_owned()))?;
                        let $field: $typ = value.as_str()
                            .parse()
                            .map_err(|_| {
                                $crate::Error::Parse(
                                    stringify!($typ).to_owned(),
                                    value.to_owned()
                                )
                            })?;
                        $field
                    },)*
                })
            }

            fn write(&self) -> Result<(), $crate::Error> {

                let mut file = String::new();

                $(
                    file.push_str(&format!("{}={}\n",
                        stringify!($field),
                        format!("{}", self.$field)
                    ));
                )*

                std::fs::write(Self::PATH, file)?;

                Ok(())
            }

        }
    }
}




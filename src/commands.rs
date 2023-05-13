use wca::{Args, Props};

type Result = std::result::Result<(), wca::BasicError>;

pub fn import_from(args: Args, _props: Props) -> Result {
    let mut args = args.0.into_iter();
    parse_args!(args, path: String);

    println!("path {path}");

    Ok(())
}

pub fn questions_list(_args: Args, _props: Props) -> Result {
    Ok(())
}

pub fn questions_about(_args: Args, _props: Props) -> Result {
    Ok(())
}

pub fn questions_export(_args: Args, _props: Props) -> Result {
    Ok(())
}

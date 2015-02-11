macro_rules! log{
    ( $($arg:tt)* ) => ({
        // Import the Writer trait (required by write!)
        use core::fmt::Writer;
        let _ = write!(&mut ::logging::Writer::get(module_path!()), $($arg)*);
    })
}


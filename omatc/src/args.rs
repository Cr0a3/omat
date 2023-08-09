extern crate clap;

use clap::{Arg, App};

const START_CODE_DEFAULT_VALUE: &str = "main";

pub struct Args {
    pub input: String,
    pub output: String,
    pub release: bool,
    pub debug: bool,
    pub no_main: bool,
    pub bare_metal: bool,
    pub obj: bool,
    pub start_code: String,
    pub static_linking: bool,
    pub dynamic_linking: bool,
}

impl Args {
    pub fn new() -> Self {
        // basic app information
        let app = App::new("Omat's compiler")
            .setting(clap::AppSettings::ArgRequiredElseHelp)
            .version("0.0.0 unstable")
            .author("Toni Ivanovski");

        // Define the command line options
        let input_file_option = Arg::with_name("input")
            .long("input") // allow --input
            .takes_value(true)
            .help("specifies to input file")
            .required(true);

        let output_file_option = Arg::with_name("output")
            .long("output") //allows --output
            .short("o") //allows -o 
            .takes_value(true)
            .help("specifies the output file");
        
        let release_option = Arg::with_name("release")
            .long("release") //allows --release
            .help("set the output as release");

        let debug_option = Arg::with_name("debug")
            .long("debug") //allows --debug
            .help("set the output as debug");

        let no_main_option = Arg::with_name("no_main")
            .long("no_main")    //allows --no_main
            .help("sets that no main function exits");

        let bare_metal_option = Arg::with_name("bare_metal")
            .long("bareMetal") //allows --bareMetal
            .help("set no os depend stuff and removes link to std libary");

        let obj_option = Arg::with_name("obj")
            .long("obj")   //allows --obj
            .short("c")    //allows -c  (from gcc)
            .help("set the output file format to object file (.o)");


        let start_code_option = Arg::with_name("start_code")
            .long("startCode") //allows --startCode 
            .takes_value(true)
            .default_value(START_CODE_DEFAULT_VALUE)
            .help("specifies the function, in which get jumped when calling the application");

        let static_linking_option = Arg::with_name("static_linking")
            .long("static_linking") // allows --static_linking
            .help("Sets, that every libary gets staticly linked (used as default).");
        
        let dynamic_linking_option = Arg::with_name("dynamic_linking")
            .long("dynamic_linking") // allows --dynamic_linking
            .help("Sets, that every libary gets linked via dynamic linking.");

        // now add in the arguments we want to parse
        let app = app
            .arg(input_file_option)
            .arg(output_file_option)
            .arg(release_option)
            .arg(debug_option)
            .arg(no_main_option)
            .arg(bare_metal_option)
            .arg(obj_option)
            .arg(start_code_option)
            .arg(static_linking_option)
            .arg(dynamic_linking_option);

        // extract the matches
        let matches = app.get_matches();
        
        let in_file = matches.value_of("input").unwrap();

        let out_file: &str;

        if matches.is_present("output") == false {
            let in_file_without_ext = match in_file.clone().rfind('.') {
                Some(index) => &in_file.clone()[0..index],
                None => in_file.clone(),
            };

            out_file = in_file_without_ext.clone();
        }
        else {
            out_file = matches.value_of("output").unwrap();
        }

        let mut release_bool: bool =  matches.is_present("release");
        let mut debug_bool: bool =  matches.is_present("debug");

        if release_bool == true && debug_bool == true {
            eprintln!("Error: can't use release and debug version at the same time. Procced with debug version.");
            release_bool = false;
            debug_bool = true;
        }

        else if release_bool == false && debug_bool == false {
            release_bool = false;
            debug_bool = true;
        }

        let no_main: bool = matches.is_present("no_main");
        let bare_metal: bool = matches.is_present("bare_metal");
        let obj: bool = matches.is_present("obj");

        
        let start_code = matches.value_of("start_code").unwrap();

        let mut static_linking: bool = matches.is_present("static_linking");
        let mut dynamic_linking: bool = matches.is_present("dynamic_linking");

        if static_linking == true && dynamic_linking == true {
            eprintln!("Error: can't use static and dynamic linking at the same time. Procced with static linking version.");
            static_linking = true;
            dynamic_linking = false;
        }

        else if static_linking == false && dynamic_linking == false {
            static_linking = true;
            dynamic_linking = false;
        }

        Args {  input: in_file.to_string(), 
                output: out_file.to_string(),
                release: release_bool.clone(),
                debug: debug_bool.clone(),
                no_main: no_main,
                bare_metal: bare_metal,
                obj: obj,
                start_code: start_code.to_string(),
                static_linking: static_linking.clone(),
                dynamic_linking: dynamic_linking.clone(),
            }
    }
}

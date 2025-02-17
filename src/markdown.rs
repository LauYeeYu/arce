use pulldown_cmark::{Parser, Options, html};
use std::fs::{File,read_to_string};
use std::io::Write;

pub fn render(src: &str, dst: &str){
    let md_str = read_to_string(&src)
        .expect("Cannot read markdown src");
    
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&md_str, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    let mut f =File::create(&dst)
            .expect(format!("Could not create file: ").as_str());

    f.write_all(html_output.as_bytes())
        .expect(format!("Could not write to file: ").as_str());

    println!("Markdown Rendered: {} to {}", src, dst);
}

pub fn render_str_to_string(md_str: &str) -> String{
    
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH); 
    let parser = Parser::new_ext(&md_str, options);
    let mut output = String::new();
    html::push_html(&mut output, parser);
    output
}

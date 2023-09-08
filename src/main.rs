use std::error::Error;
use std::{process::exit};
use std::io;
use xml::reader::{EventReader, XmlEvent};
use std::fs::{File,self};
use std::path::Path;
use std::collections::HashMap;

#[derive(Debug)]
struct  Lexer<'a>{
    content: &'a [char],
}

impl<'a>  Lexer<'a> {
    fn new(content:&'a [char])->Self {
        Self { content }
    }

    fn trim_left(&mut self){

         while self.content.len() > 0 && self.content[0].is_whitespace(){
            self.content = &self.content[1..];
        }

    }

    // Remove and chop off some characters
    fn chop(&mut self, n:usize)->&'a[char]{
        let token = &self.content[0..n];
        self.content = &self.content[n..];

        token
    }

    //Remove and chopmoff some characters while the predicate(the character is of type)
    fn chop_while<P>(&mut self, mut predicate:P)-> &'a [char] where P:FnMut(&char)->bool{
        let mut n = 0;
        while n < self.content.len() && predicate(&self.content[n]){
            n +=1;
        }
        self.chop(n)
    }

    fn next_token(&mut self)-> Option<&'a [char]>{
       
        self.trim_left();
        if self.content.len() == 0{
            return None
        }

        if self.content[0].is_numeric(){
          
            return Some(self.chop_while(|x| x.is_numeric()))
        }

        if self.content[0].is_alphabetic(){
          return Some(self.chop_while(|x| x.is_alphanumeric()))
        }

      
        return Some(self.chop(1));

        // todo!("Invalid token start with {start}",start = self.content[0])
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = &'a[char];

    fn next(&mut self)->Option<Self::Item>{
        self.next_token()
    }
}

// fn index_document(doc_content:&str)->HashMap<String, usize>{
//     todo!()
// }

fn read_xml_files<P:AsRef<Path>>(file_path:P)->io::Result<String>{

    let file = File::open(file_path)?;
    let er = EventReader::new(file);


    let mut content = String::new();
    
    for event in er.into_iter(){

        if let XmlEvent::Characters(text) = event.expect("TODO"){
           

            content.push_str(&text);
            content.push_str(" ");
        };
    }

    Ok(content)
}
fn main()-> io::Result<()> {
    // let file_path = "c:/xampp/htdocs/slurstudio/resources/views/welcome.blade.php";
    // let file_path = "docs.gl/gl4/glClear.xhtml";
    // Split the file by one . from the right path 
    // file_path.rsplit_once('.');
  
    // println!("{content}",content = read_xml_files(file_path).expect("TODO"));

    let content = read_xml_files("docs.gl/gl4/glVertexAttribBinding.xhtml")?
        .chars()
        .collect::<Vec<_>>();
    let lexer = Lexer::new(&content);
    for token in lexer{
        println!("{token}",token = token.iter().collect::<String>());
    }
    //Read file directly
    //Read and put every charater in a vector
    // let content = read_xml_files("docs.gl/gl4/glVertexAttribBinding.xhtml")?.chars().collect::<Vec<_>>();
    // let lexer = Lexer::new(&content);
    // println!("{lexer:?}");

    // let dir_path = "docs.gl/gl4";
    // let dir  = fs::read_dir(dir_path)?;


    // for file in dir{
    //     let file_path = file?.path();
    //     let content = read_xml_files(&file_path)?;

    //     println!("{file_path:?}=> {size}",size= content.len());
     
    // }

    Ok(())

}
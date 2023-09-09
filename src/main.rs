// use std::error::Error;
// use std::{process::exit};
use std::io;
use xml::reader::{EventReader, XmlEvent};
use std::fs::{File,self};
use std::path::{Path,PathBuf};
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
type TermFreq = HashMap<String,usize>;
type TermFreqIndex = HashMap<PathBuf,TermFreq>;
fn main()-> io::Result<()> {
    // let file_path = "c:/xampp/htdocs/slurstudio/resources/views/welcome.blade.php";
    // let file_path = "docs.gl/gl4/glClear.xhtml";
    // Split the file by one . from the right path 
    // file_path.rsplit_once('.');
    // println!("{content}",content = read_xml_files(file_path).expect("TODO"));

    
    let dir_path = "docs.gl/gl4";
    let dir  = fs::read_dir(dir_path)?;
    let take_number = 20;
    let mut tf_index = TermFreqIndex::new();

    for file in dir{
        let file_path = file?.path();

        println!("indexing {:?}...", &file_path);
        // let content = read_xml_files(&file_path)?;
        // let file_path = "docs.gl/gl4/glVertexAttribBinding.xhtml";

        let content = read_xml_files(&file_path)?
            .chars()
            .collect::<Vec<_>>();

        //Declare a HashMap to store each term and its number of occurences
        let mut term_frequency = TermFreq::new();

        let lexer = Lexer::new(&content);
        for token in lexer{
            //Convert all terms to lower case
            let term = token.iter().map(|x| x.to_ascii_lowercase()).collect::<String>();

            //Count the number of occurence of each term then put each equivalent in a HashMap
            if let Some(freq) = term_frequency.get_mut(&term)  {
                *freq +=1;
            }
            else {
                term_frequency.insert(term, 1);
            }

            // println!("{:?}", term_frequency);
            // println!("{token}",token = token.iter().collect::<String>());
        }

        let mut stats = term_frequency.iter().collect::<Vec<_>>();
        stats.sort_by_key(|(_,f)| *f );//This will sort in ascending order(From lowest to the highest)
        stats.reverse();//This will reverse the sort from Highest to the lost

        tf_index.insert(file_path, term_frequency);
        // println!("{file_path:?} :");
        // for (term, frequency) in stats.iter().take(take_number){
        //     println!("{term}=> {frequency}")
        // }
    
        // println!("{file_path:?}=> {size}",size= content.len());
    }

    for (path,tf) in tf_index{
        println!("{path:?} has {count} unique tokens", count = tf.len());
    }
    //Read file directly
    //Read and put every charater in a vector
    // let content = read_xml_files("docs.gl/gl4/glVertexAttribBinding.xhtml")?.chars().collect::<Vec<_>>();
    // let lexer = Lexer::new(&content);
    // println!("{lexer:?}");

   

    Ok(())

}

use std::io::prelude::*;
use std::path::PathBuf;
use std::error::Error;
use std::fs::File;
use std::string::String;
use std::fs::OpenOptions;

pub trait PrintUtilities{
    fn Print( &self, stuff2Print: &String );
    fn PrintAndNewPar( &self, stuff2Print: &mut String ){
        self.AddNewLineChar(stuff2Print);
        self.Print(stuff2Print);
    }

    fn AddNewLineChar( &self,basicString: &mut String){
        basicString.push_str("\n");
    }
}

pub struct FilePrinter{
    filePath: PathBuf,
}

impl PrintUtilities for FilePrinter{
    fn Print( &self, stuff2Print: &String ){
        let display = self.filePath.display();
        
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(self.filePath.as_path())
            .unwrap();

        match file.write_all(stuff2Print.as_bytes()){
            Err(why) => panic!("couldn't write to {}: {}", display, why.to_string()),
            Ok(_) => {},
        }
    }
}

pub struct ScreenPrinter{
    
}

impl PrintUtilities for ScreenPrinter{
    fn Print( &self, stuff2Print: &String ){
        print!("{}", stuff2Print);
    }
}
fn main(){
    let path = PathBuf::from("prova.txt");
    let filePrinter = FilePrinter{filePath: path};
    let screenPrinter = ScreenPrinter{};

    let colLimit: u8 = 12;

    for index in 1 .. 145 {
        let mut string2Print = String::from(" ");
        let indexVal = index as u8;
        string2Print.push_str(&indexVal.to_string());

        if( indexVal % colLimit) != 0{
            filePrinter.Print(&string2Print);
            screenPrinter.Print(&string2Print);
        }
        else{
            filePrinter.PrintAndNewPar(&mut string2Print);
            screenPrinter.PrintAndNewPar(&mut string2Print);
        }
        
    }
}
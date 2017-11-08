fn try(e:Vec<String>,f:Vec<String>,i:i32,j:i32){

    let  N = e.len(); 
    let  M = f.len(); 
    let  L = e.len() + f.len(); 
    let  Z = 2 * cmp::min(e.len(),f.len())+2;

    if N > 0 && M > 0 {

        let w = N-M; 
        let g = vec![0;Z];
        let p = vec![0;Z];



    }

}


use std::vec;
use std::cmp;
use std::ops;
use std::ops::{Mul, Range};
use std::env;

use std::error::Error;



use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

fn lines_from_file<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}




pub trait ModuloSignedExt {
    fn modulo(&self, n: Self) -> Self;
}
macro_rules! modulo_signed_ext_impl {
    ($($t:ty)*) => ($(
        impl ModuloSignedExt for $t {
            #[inline]
            fn modulo(&self, n: Self) -> Self {
                (self % n + n) % n
            }
        }
    )*)
}
modulo_signed_ext_impl! { i8 i16 i32 i64 }

struct MulStep<T> {
    step: T,
    pos: T,
    end: T,
}

impl<T> Iterator for MulStep<T>
where
    T: Copy + Ord + Mul<T, Output = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.end {
            let result = self.pos;
            self.pos = self.pos * self.step;
            Some(result)
        } else {
            None
        }
    }
}

trait StepByMulExtender<T> {
    fn step_by_mul(self, step: T) -> MulStep<T>;
}

impl<T> StepByMulExtender<T> for Range<T> {
    fn step_by_mul(self, step: T) -> MulStep<T> {
        MulStep {
            step: step,
            pos: self.start,
            end: self.end,
        }
    }
}


fn multiLineString_ToVector(input : &str) -> Vec<Vec<String>> {
    let mut output:Vec<Vec<String>> = vec![]; 
    
    let mut stringTest = input;
    let mut split = stringTest.split("\n");
    let mut vector1:Vec<&str> = vec![]; 
    let mut vectorTaker:Vec<Vec<String>> = vec![];
    for s in split {
        let mut splitA = s.split(" ");
        vector1 = splitA.collect();
        let mut to_beAdded:Vec<String> = vec![];
        for i in 0..vector1.len() {
           
            to_beAdded.push(vector1[i].to_string());
            
        }
        vectorTaker.push(to_beAdded);
    }
    
    
    return vectorTaker;
}


fn multiLineString_ToVector_SplitBySpace(input : &str) -> Vec<String>  {

    let mut output:Vec<String> = vec![];
    for s in input.split(" "){
        output.push(s.to_string());        
    }
    
    
    return output;
}
fn multiLineString_ToVector_SplitByNewLine(input : &str) -> Vec<String>  {

    let mut output:Vec<String> = vec![];
    for s in input.split("\n"){
        output.push(s.to_string());        
    }
    
    
    return output;
}

fn multiLineString_ToVector_SplitBySpaceString(input : &String) -> Vec<String>  {

    let mut output:Vec<String> = vec![];
    for s in input.split(" "){
        output.push(s.to_string());        
    }
    
    
    return output;
}


fn printAllVectorOfVector(input:Vec<Vec<String>>){
    
    for i in 0..input.len(){
        println!("-------------------- New Line --------------------");
        
        println!("Value => {:?}",input[i]); 
        
        println!("  -------------------- End of Sentence  --------------------");
    }
}

// 2 two files 

fn main() {

    
    let  mut test1 = lines_from_file("C:/Users/Abhishek Shankar/Desktop/Uni/Project/Rust Project/Rust Code/Differencer/src/test1.txt");
    let mut test2 = lines_from_file("C:/Users/Abhishek Shankar/Desktop/Uni/Project/Rust Project/Rust Code/Differencer/src/test2.txt");

    let s: String = test1.to_vec().into_iter().collect();
    let s2: String = test2.to_vec().into_iter().collect();
    let mut result = diff(&mut test1, &mut test2, 0, 0);

    appyScriptApplication(result,test1.to_vec(),test2.to_vec()); 

}




fn getValue_FromArray(arry:&mut Vec<String>,mut index: i32)-> String  {
    
    let arrylen:i32 = arry.len() as i32; 
    
    if(index < 0 && (index*-1) < arrylen){
        let mut st1  = "NULL";
        let mut new_index = arrylen-(index*-1); 
        let str2 = &arry[new_index as usize].to_string();
        return str2.to_string();
        
       
    }else if(index+1 > arrylen){
        return "".to_string(); 
    }
    return arry[index as usize].to_string();
        
}

fn diff(string1: &mut Vec<String>, string2: &mut Vec<String>, mut i: u32, mut j: u32) -> Vec<ReturnValue> {

    if i == 0 && j == 0 {
        i = 0;
        j = 0;
    }
    
    let mut n: i32 = string1.len() as i32;
    let mut m: i32 = string2.len() as i32;
    let mut l: i32 = (string1.len() + string2.len()) as i32;
    let mut z: i32 = (2 * cmp::min(string1.len(), string2.len()) + 2) as i32;


    if n > 0 && m > 0 {
        let mut w = n - m;
        let mut g = vec![0; z as usize];
        let mut p = vec![0; z as usize];
        
        let mut value: i32 = l;
        value = (value / 2 as i32);

        if l % 2 != 0 {
            value = value + 1;
        }
        value += 1;
        
        for h in 0..value {
            for r in 0..2 {
                
                let mut giveCValueToG = false;      
                let mut c = p.clone();
                let mut d = g.clone();
                let mut o = 0;
                let mut m2: i32 = -1;
                if r == 0 {
                    giveCValueToG = true;
                    c = g.clone();
                    d = p.clone();
                    o = 1;
                    m2 = 1 as i32;
                }
                
                
              
               
                let mut from_1: i32 = ((-(h - 2 * cmp::max(0, (h - m)))) as i32);
                let mut to_1: i32 = ((h - 2 * cmp::max(0, h - n) + 1) as i32); //, 2);
                let mut step: i32 = 2;
                let mut k: i32 = from_1;
                while k <= to_1 {
                
                    let index1 = (k - 1).modulo(z as i32);
                    let check1_1 = -(h) as i32;
                    let check1 = k == check1_1 as i32;
                    let check2 = h as i32;
                    let mut check3 = (k-1).modulo(z);
                    let mut check4 = (k+1).modulo(z);
                    let index3 = (k - 1).modulo(z as i32);
                    let mut  a = c[index3 as usize] +1;
                    let index2 = (k + 1).modulo(z as i32);
                    if check1 || (k != check2) && c[index2 as usize] < c[index3 as usize] {
                        a = c[index2 as usize];
                        
                    }
                    

                    let mut b = a - (k as i32);
                    let mut s = a;
                    let mut t = b;
                    let mut indexString1: i32 = ((1 - o) * n + m2 * a + (o - 1)) as i32;
                    let mut indexString2: i32 = ((1 - o) * m + m2 * b + (o - 1)) as i32;
                    
                    while a < (n) && b < (m) && getValue_FromArray(string1,indexString1) == getValue_FromArray(string2,indexString2) {
                        
                        a = a + 1;
                        b = b + 1;
                        indexString1 = ((1 - o) * n + m2 * a + (o - 1)) as i32;
                        indexString2 = ((1 - o) * m + m2 * b + (o - 1)) as i32;
                    }    
                       
                       
                        let substraction = k - (w as i32);
                        let mut z1: i32 = -(substraction as i32);
                        let index1: usize = ((k).modulo(z as i32)) as usize;
                        
                        c[index1 as usize] = (a as i32);
                        
                        if(giveCValueToG){
                            
                            g = c.clone();
                            p = d.clone(); 
                            
                        }else {
                            p = c.clone();
                            g = d.clone(); 
                            
                            
                           
                        }
                         if l.modulo(2) == o && z1 >= -(h - o) && z1 <= (h - o)
                            && c[(k.modulo(z as i32)) as usize] + d[(z1.modulo(z)) as usize] >= n
                        {
                            
                                let mut D = 2 * h;
                                let mut x = n - a;
                                let mut y = m - b;
                                let mut u = n - s;
                                let mut v = m - t;
                            if o == 1 {
                                
                                D = 2 * h - 1;
                                x = s;
                                y = t;
                                u = a;
                                v = b;
                            }
                              if D > 1 || (x != u && y != v) {
                                
                                let r1_1: usize = x as usize;
                                let r2_1: usize = y as usize;
                                let r3_1: u32 = i as u32;
                                let r4_1: u32 = j as u32;
                               
                               
                               
                               
                               
                                let r5_1: usize = u as usize;
                                let r5_2: usize = n as usize;
                                let r6_1: usize = v as usize;
                                let r6_2: usize = m as usize;
                                let r7_1: u32 = i as u32;
                                let r8_1: u32 = j as u32;
                               
                                let mut recursion1:Vec<ReturnValue> = diff(
                                    &mut string1[0..r1_1].to_vec(),
                                    &mut string2[0..r2_1].to_vec(),
                                    r3_1,
                                    r4_1,
                                );
                     
                                let mut recursion2:Vec<ReturnValue> = diff(
                                    &mut string1[r5_1..r5_2].to_vec(),
                                    &mut string2[r6_1..r6_2].to_vec(),
                                    r7_1+ (u as u32),
                                    r8_1 + (v as u32),
                                );
                                return  vectorAdd_Up(recursion1,recursion2);
                                
                               
                            }
                            else if m > n {
                                let r1_1: usize = n as usize;
                                let r1_2: usize = m as usize;
                                let r2_1: u32 = (i + (n as u32)) as u32;
                                let r2_2: u32 = (j + (n as u32)) as u32;
                                
                               
                                return vectorAdd_Up(
                                    diff(&mut string1[0..0].to_vec(),
                                        &mut string2[r1_1..r1_2].to_vec(),
                                        r2_1,
                                        r2_2)
                                ,vec![]);
                                        
                                
                                
                                
                            }
                            else if m < n 
                             {
                                let r1_1: usize = m as usize;
                                let r1_2: usize = n as usize;
                                let r3_1: u32 = (i + (m as u32)) as u32;
                                let r4_1: u32 = (j + (m as u32)) as u32;
                                
                                return vectorAdd_Up(diff(
                                    &mut string1[r1_1..r1_2].to_vec(),
                                    &mut string2[0..0].to_vec(),
                                    r3_1,
                                    r4_1,
                                    ), vec![]);
                               
                            }
                            else {
                                
                                
                                let output:Vec<ReturnValue> = vec![];
                                return vectorAdd_Up(vec![],vec![]);
                            }
                        }
                        
                        indexString1 = ((1 - o) * n + m2 * a + (o - 1)) as i32;
                        indexString2 = ((1 - o) * m + m2 * b + (o - 1)) as i32;
                     
                    k += step;
                    
                }
                
                
                
            }
           
        }
        let output:Vec<ReturnValue> = vec![];
        return output;
         
    } else if n > 0 {
        let mut stringoutput = "";
        let mut output:Vec<ReturnValue> = Vec::new();
        for x in 0..n {
            
            let r1 = format!("{}", (i+x as u32));
            let  returnVal = ReturnValue{
                operation: String::from("delete"),
                position_old: String::from(r1.to_string()),
                position_new: String::from("NULL"),        
            };
            output.push(returnVal);
            
        }
      
        
       
        return output;
    } else {
        
        let mut output:Vec<ReturnValue> = Vec::new();
        for x in 0..m {
            
            let r1 = format!("{}", (i as u32));
            let r2 = format!("{}", (j+x as u32));
            let  returnVal = ReturnValue{
                operation: String::from("insert"),
                position_old: String::from(r1.to_string()),
                position_new: String::from(r2.to_string()),        
            };
            output.push(returnVal);
            
        }

        return output;
    }
}

fn appyScriptApplication(edit_script:Vec<ReturnValue>,s1:Vec<String>,s2:Vec<String>){
    let mut i =0; 
    let mut new_sequence = vec![];

    for e in edit_script{
        while e.position_old.to_string().parse::<usize>().unwrap() > i {
            new_sequence.push(s1[i].to_string());
            println!("   {}", s1[i].to_string());
            i +=1;
            
        }

        if e.position_old.to_string().parse::<usize>().unwrap() == i {
            if e.operation.to_string() == "delete" {
                println!("-  {}", s1[i].to_string());
                i  = i+1;
                
            }
            else if e.operation.to_string() == "insert" {
                new_sequence.push(s2[e.position_new.to_string().parse::<usize>().unwrap()].to_string());
                println!("+  {}", s2[e.position_new.to_string().parse::<usize>().unwrap()].to_string());
            }
        }
    }
    while i < s1.len(){
        new_sequence.push(s1[i].to_string());
        println!("{}", s1[i].to_string());
        i+=1;
    }
}


fn printValueFromIndex(vector:Vec<String>,from:usize,to:usize){
    for i in from..to {
        println!("{:?}",vector[i]);
    }
}




fn vectorAdd_Up_V2(v1:Vec<String>,v2:Vec<String>) -> Vec<String> {
    
    let mut output:Vec<String> = vec![]; 
    
    for i in 0..v1.len() {
        output.push(v1[i].to_string());
    }
    
    for i in 0..v2.len() {
        output.push(v2[i].to_string());
    }
    
    return output;
    

}

fn vectorAdd_Up(a:Vec<ReturnValue>,b:Vec<ReturnValue>) -> Vec<ReturnValue> {
    let mut addVector:Vec<ReturnValue> = vec![]; 

    for i in 0..a.len(){
        let out:ReturnValue = ReturnValue{
                                operation   : a[i].operation.to_string(),
                                position_old: a[i].position_old.to_string(),
                                position_new: a[i].position_new.to_string(),
                                };
        addVector.push(out);            
    }
    
    for i in 0..b.len(){
        let out:ReturnValue = ReturnValue{
                                operation   : b[i].operation.to_string(),
                                position_old: b[i].position_old.to_string(),
                                position_new: b[i].position_new.to_string(),
                                };
        addVector.push(out);            
    }
    
    return addVector;
    
    

}


struct ReturnValue {
    operation: String,
    position_old: String,
    position_new: String,
}


fn print_out_ReturnValue(output:Vec<ReturnValue>){
    for i in 0..output.len(){
        println!("PRINT => {}",output[i]);        
    }
}


impl std::fmt::Display for ReturnValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(operation => : {}) | (position_old => {}) | (position_new => {})",
        self.operation,self.position_old,self.position_new)
    }
}

fn adding_dataObj_Vector(a:ReturnValue, b:ReturnValue)->Vec<ReturnValue>{
    let mut addVector:Vec<ReturnValue> = vec![a,b];
    return addVector;
   
}









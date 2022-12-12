use std::error::Error;
use std::io;
use std::process;
use csv::Reader;
use csv::StringRecord;
extern crate csv;
extern crate rustc_serialize;
use serde::Deserialize;
use std::fs::File;
use std::io::BufRead;
use std::collections::BTreeMap;



#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Rec { 
    index: u32,
    work_year: u32,
    experience_level: String,
    employment_type: String,
    job_title: String,
    salary: u32,
    salary_currency: String,
    salary_in_usd: u32,
    employee_residence: String,
    remote_ratio: u32,
    company_location: String,
    company_size: String


}

fn open_file(path: &str) -> Result<(), Box<dyn Error>> {
    // this just opens a CSV file and takes all the data into a struct
    let file = File::open(path).expect("Didn't work");
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() { 
        let record: Rec = result?;
    }
    Ok(())
}

fn check_csv(path: &str) { 
    // this is to check if the function above worked as intended
    if let Err(err) = open_file(path) {
        println!("{} didn't work",err);
        process::exit(1);
    }
}

fn read_file(path: &str) -> Vec<(u32,u32,String)> { 
    // This function  takes in a file and returns a vector of tuples
    // in the specific data set that I am working in, I only care about 3 variables
    // the index, the job title, and the salary in USD, so that is what is inside the vectors
    let mut vec = Vec::new();
    let file = File::open(path).expect("Didn't work");
    let buf_reader = std::io::BufReader::new(file).lines();
    let mut count = 1; 
    for line in buf_reader { 
        if count == 1 {
            count += 1;
            continue 
        }else {
            let line_str = line.expect("Couldn't read");
            let v: Vec<&str> = line_str.trim().split(",").collect();
            let a = v[0].parse::<u32>().unwrap();
            let e = v[4].parse::<String>().unwrap();
            let h = v[7].parse::<u32>().unwrap();
            vec.push((a,h,e));

        }
    }return vec
}

fn graph(vec:Vec<(u32,u32,String)>) -> Vec<(String,String)> { 
    // Takes in a vector and returns another vector of tuple of strings
    // where the extra string is the range in which the salary is in 
    // ranges will be specified in the function below. The value represents maximum range
    // the output of this function will be a graph, with the first element is the node and the
    // second is the node it is connected to 
    let range1 = 40000;
    let range2 = 80000;
    let range3 = 120000;
    let range4 = 160000;
    let range5 = 200000;
    let range6 = 240000;
    // there will be a range7 but it will be if it's greater than range6 
    let mut new_vec: Vec<(String,String)> = Vec::new();
    for i in 0..vec.len() {
        if vec[i].1 <= range1 {
            let range = "<$40,000";
            new_vec.push((vec[i].2.to_string(),range.to_string()));
        }else if vec[i].1 > range1 && vec[i].1 <= range2 {
            let range = "$40,000-$80,000";
            new_vec.push((vec[i].2.to_string(),range.to_string()));
        }else if vec[i].1 > range2 && vec[i].1 <= range3 {
            let range = "$80,000-$120,000";
            new_vec.push((vec[i].2.to_string(),range.to_string()));
        }else if vec[i].1 > range3 && vec[i].1 <= range4 {
            let range = "$120,000-$160,000";
            new_vec.push((vec[i].2.to_string(),range.to_string()));
        }else if vec[i].1 > range4 && vec[i].1 <= range5 {
            let range = "$160,000-$200,000";
            new_vec.push((vec[i].2.to_string(),range.to_string()));
        }else if vec[i].1 > range5 && vec[i].1 <= range6 {
            let range = "$200,000-$240,000";
            new_vec.push((vec[i].2.to_string(),range.to_string()));
        }else {
            let range = "$240,000+";
            new_vec.push((vec[i].2.to_string(),range.to_string()));
        }

           
    }
    return new_vec

}

fn connected_to<'a>(graph:&'a Vec<(String,String)>,salary:&'a String) -> Vec<&'a String> {
    // This functions takes in a graph and finds all the jobs that are connected to the input salary 
    let mut list = BTreeMap::new();
    for node in graph {
        if &node.1 == salary {
            match list.get(&node.0) {
                Some(val) => continue,
                None => list.insert(&node.0,0)
            };
        }else {
            continue
        }
    }
    let mut treevec = Vec::from_iter(list);
    let mut new_treevec = Vec::new();
    for job in treevec {
        new_treevec.push(job.0);
    }
    return new_treevec
    
}


fn weight(graph:&Vec<(String,String)>,job:&String,salary:&String) -> f32 {
    // This function takes in a graph and returns a weight to job title associated with the salary range
    let mut num_job = 0.0;
    let mut count = 0.0;
    for node in graph {
        if &node.1 == salary {
            if &node.0 == job { 
                count += 1.0;
                num_job += 1.0;
            }else {
                num_job += 1.0;
            }
        }else {
            continue 
        }
    }
    return count/num_job
    
}

fn highest_weight(graph:Vec<(String,String)>,salary:&String) -> String {
    // For a given range, the function returns the job with the highest weights
    let mut jobs = connected_to(&graph,&salary);
    let mut tree = BTreeMap::new();
    for job in &jobs {
        tree.insert(job,weight(&graph,&job,&salary));
    }
    let mut treevec = Vec::from_iter(&tree);
    let mut rank = Vec::new();
    for elem in &treevec {
        rank.push(elem.1)
    }
    rank.sort_by(|a, b| a.partial_cmp(b).unwrap());
    rank.reverse();
    let mut output = String::new();
    for &job in &jobs {
        if &&tree[&job] == &rank[0] {
            output = job.to_string();
        }else {
            continue
        }
    }return output

}

fn predict_salary() {
    // for a given job, the function will predict the salary using random 
}
fn most_similar() {
    // takes in a job and prints the most similar job based on salary weights 
}

fn main() {
	let file = "/Users/ryanice/Desktop/DS HW/DS Project/Project/src/ds_salaries.csv";
    open_file(file);
    let mut vec = read_file(file);
    let mut vec1 = graph(vec);
    //println!("{:?}",vec1);
    // print out all the jobs with the associated salary and weights, print out which weights 
    //let mut test = weight(vec1,"Machine Learning Engineer".to_string(),"$160,000-$200,000".to_string());
    //println!("{}",test);
    //connected_to(vec1,"$160,000-$200,000".to_string());
    highest_weight(vec1,&"$160,000-$200,000".to_string());
}
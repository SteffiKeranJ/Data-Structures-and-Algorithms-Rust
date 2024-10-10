/**
https://codeforces.com/problemset/problem/1553/B
**/

use std::io::{self, BufRead};

fn main() -> io::Result<()> {
  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();
  let tc:i32 = lines.next().unwrap()?.parse().unwrap();
  let mut result = String::new();
  for _ in 0..tc {
    let s: Vec<char> = lines.next().unwrap()?.chars().collect();
    let t: Vec<char> = lines.next().unwrap()?.chars().collect();
    let mut flag = false;
    for i in 0..s.len() {
      for j in 0..t.len() {
        if s[i] != t[j] {
          continue;
        } 
        let mut s_count = 0;
        while i >= s_count && j >= s_count && s[i-s_count]==t[j-s_count] {
          s_count+=1;
        }
      
        let mut e_count = 1;
        while i >=e_count && j+e_count < t.len() && s[i-e_count]==t[j+e_count] {
          e_count+=1;
        }
        if s_count+e_count-1 == t.len() {
          flag = true;
        }        
      }
    }
    result.push_str(if flag{
      "YES\n"
    } else {
      "NO\n"
    });
    
  }
  print!("{}", result);
  Ok(())
    
}

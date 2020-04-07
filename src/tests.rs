use std::fs::File;
use std::io::prelude::*;

#[test]
fn parse_xlsx() {
  let mut content: Vec<u8> = Vec::new();
  match File::open("./src/test.xlsx").and_then(|mut f| {
    f.read_to_end(&mut content)
  }) {
    Ok(_) => {
      match super::parse_xlsx(&content, Some([3].to_vec())) {
        Ok(tables) => {
          let table = tables[0].clone();
          let ref row1 = table[&2];
          assert_eq!(row1[&2], "Rust");
          assert_eq!(row1[&0], "Test 1");
          assert_eq!(row1[&3], "2015-05-15");
          let ref row2 = table[&3];
          assert_eq!(row2[&2], "Emma");
          assert_eq!(row2[&3], "2014-07-04");
          let ref row3 = table[&4];
          assert_eq!(row3[&2], "Nikita");
          assert_eq!(row3[&3], "2002-10-08");
        },
        Err(err) => panic!(err)
      }
    }
    Err(_) => panic!("Test file not found")
  }
}


#[test]
fn parse_ny_xlsx() {
  let mut content: Vec<u8> = Vec::new();
  match File::open("./src/ny_test.xlsx").and_then(|mut f| {
    f.read_to_end(&mut content)
  }) {
    Ok(_) => {
      match super::parse_xlsx(&content, Some([3].to_vec())) {
        Ok(tables) => {
          for table in tables.iter(){
            for (index,row) in table{
              println!("first cell {}",row[&0] );
            }
          }
          /* let ref row1 = table[&2];
          assert_eq!(row1[&2], "Rust");
          assert_eq!(row1[&0], "Test 1");
          assert_eq!(row1[&3], "2015-05-15");
          let ref row2 = table[&3];
          assert_eq!(row2[&2], "Emma");
          assert_eq!(row2[&3], "2014-07-04");
          let ref row3 = table[&4];
          assert_eq!(row3[&2], "Nikita");
          assert_eq!(row3[&3], "2002-10-08"); */
        },
        Err(err) => panic!(err)
      }
    }
    Err(_) => panic!("Test file not found")
  }
}

//#[test]
//fn debug() {
//  let mut content: Vec<u8> = Vec::new();
//  match File::open("./src/debug.xlsx").and_then(|mut f| {
//    f.read_to_end(&mut content)
//  }) {
//    Ok(_) => {
//      match super::parse_xlsx(&content, None) {
//        Ok(table) => panic!(format!("{:?}", table)),
//        Err(err) => panic!(err)
//      }
//    }
//    Err(_) => panic!("Test file not found")
//  }
//}

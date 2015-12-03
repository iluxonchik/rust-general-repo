fn compute_kmp_table(word: &String) -> Vec<i32> {
    let mut table : Vec<i32> = vec![0; word.len()];
    let mut pos = 2;
    let mut cnd = 0;

    let word_chars : Vec<char> = word.chars().collect::<Vec<char>>();

    table[0] = -1;
    table[1] = 0;
    
    while pos < word.len() {
        if word_chars[pos - 1] == word_chars[cnd] {
            table[pos] = (cnd + 1) as i32;
            cnd += 1;
            pos += 1;
        } else if cnd > 0 {
            cnd = (table[cnd]) as usize;
        } else {
            table[pos] = 0;
            pos += 1;
        }
    }

    table
}

fn kmp_serch(word: & String, text: & String) -> usize {
    let mut m : usize = 0;
    let mut i : usize = 0;
    let table : Vec<i32> = compute_kmp_table(&word);
    let text_len : usize = text.len();
    let word_len : usize = word.len();

    let word_chars : Vec<char> = word.chars().collect::<Vec<char>>();
    let text_chars : Vec<char> = text.chars().collect::<Vec<char>>();

    while m + i < text_len {
        if word_chars[i] == text_chars[m + i] {
            if i == word_len - 1 {
                return m;
            }
            i += 1;
        } else {
            if table[i] > -1 {
                m += i - (table[i] as usize);
                i = table[i] as usize;
            } else {
                i = 0;
                m += 1;
            }
        }
    }
    text_len // no match found
}


#[test]
fn compute_kmp_table_test_1() {
    let word : String = "ABCDABD".to_string();
    let expected_res : Vec<i32> = vec![-1, 0, 0, 0, 0, 1, 2];
    let res : Vec<i32> = compute_kmp_table(&word);
    
    assert_eq!(res.len(), expected_res.len());

    for i in 0..res.len() {
        assert_eq!(res[i], expected_res[i]);
    }
}

#[test]
fn kmp_serch_1() {
    let word : String = "ABCDABD".to_string();
    let text : String = "ABC ABCDAB ABCDABCDABDE".to_string();
    let expected_res : usize = 15;

    let res : usize = kmp_serch(&word, &text);
    
    assert_eq!(res, expected_res);

}
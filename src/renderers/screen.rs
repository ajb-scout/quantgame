pub fn display_result_summary(ans: &Vec<MathAnswer>) -> Paragraph {
    let title: Title = Title::from("Results");

    let grouped_sums: HashMap<Sign, i32> = ans.into_iter().fold(HashMap::new(), |mut acc, item| {
        *acc.entry(item.q.sign).or_insert(0) += 1;
        acc
    });

    let mut line_vec = vec![];
    line_vec.push(Line::from(format!("Score: {}", ans.len() - 1)));

    for (s, i) in grouped_sums {
        line_vec.push(Line::from(format!["{:<8}: {}", s.to_string(), i]));
    }

    Paragraph::new(line_vec).block(Block::bordered().title(title))
}



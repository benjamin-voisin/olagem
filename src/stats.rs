use crate::app::AppResult;

use ratatui::prelude::*;
use ratatui::widgets::*;



#[derive(Debug, serde::Deserialize)]
struct TestResult {
    langage: String,
    time: f64,
    keys: u64,
}

fn generate_datasets() -> AppResult<(Vec<(usize, String)>, Vec<Vec<(f64, f64)>>)> {
    let results_path = dirs::config_dir()
        .unwrap()
        .join("olagem/results.csv");
    let mut rdr = csv::Reader::from_path(results_path)?;


    let mut feur: Vec<(usize, String)> = Vec::new();
    let mut protu: Vec<Vec<(f64, f64)>> = Vec::new();

    let mut u_index = 0;
    let mut index = 1.0;

    for result in rdr.deserialize() {
        let record: TestResult = result?;
        let wpm = ((record.keys / 5) as f64) * (60.0 / record.time);
        match feur.iter().find(|(_, s)| s.to_string() == record.langage) {
            Some((i, _)) => protu[i.to_owned()].push((index, wpm)),
            None => {
                feur.push((u_index, record.langage));
                u_index += 1;
                protu.push(vec![(index, wpm)])
            },
        }
        index += 1.
    }
    Ok((feur, protu))
}

fn find_max_wpm(records: &Vec<Vec<(f64, f64)>>) -> f64 {
    let mut max_wpm = 0.;
    for list in records.iter() {
        for record in list.iter() {
            if record.1 > max_wpm { max_wpm = record.1 }
        }
    }
    max_wpm
}

fn max_wpm_to_labels<'a>(max_wpm: f64) -> Vec<Span<'a>> {
    if max_wpm <= 50. {
        return vec!["0".into(), "25".into(), "50".into()]
    }
    else if max_wpm <= 100. {
        return vec!["0".into(), "50".into(), "100".into()]
    }
    else if max_wpm <= 200. {
        return vec!["0".into(), "50".into(), "100".into(), "150".into(), "200".into()]
    }
    vec!["0".into(), "100".into(), "200".into(), "300".into()]
}

pub fn render_chart(layout: Rect, frame: &mut Frame) {

    let (feur, protu) = generate_datasets().unwrap();

    let styles = [Style::default().red(), Style::default().blue()];

    let mut datasets = Vec::new();

    for dataset in feur.iter() {
        datasets.push(
            Dataset::default()
                .name(&dataset.1)
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(styles[dataset.0])
                .data(&protu[dataset.0])
        )
    }

    // Create the X axis and define its properties
    let mut n_records = 1;
    for list in protu.iter() {
        n_records += list.len()
    }
    let x_axis = Axis::default()
        .title("Time".red())
        .style(Style::default().white())
        .bounds([0.0, n_records as f64])
        .labels(vec!["".into(), "".into()]);


    // Create the Y axis and define its properties
    let y_axis = Axis::default()
        .title("WPM".red())
        .style(Style::default().white())
        .bounds([0.0, 150.0])
        .labels(max_wpm_to_labels(find_max_wpm(&protu)).into());

    // Create the chart and link all the parts together
    let chart = Chart::new(datasets)
        .block(Block::new().title("Chart"))
        .x_axis(x_axis)
        .y_axis(y_axis);

    frame.render_widget(chart, layout);
}

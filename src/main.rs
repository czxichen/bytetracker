use tracing::Object;

fn main() {
    let mut tracing = tracing::Tracing::new(30, 30);

    for (idx, val) in [1.1, 2.2, 3.3, 4.4].iter().enumerate() {
        let rect = tracing::cv_Rect_ {
            x: 1.0,
            y: *val,
            width: 100.0,
            height: 200.0,
            ..Default::default()
        };

        let objects = [Object {
            rect,
            label: idx as i32,
            prob: 0.9,
        }];

        for i in tracing.update(&objects) {
            println!("{idx} --> {:?}", i);
        }
    }
}

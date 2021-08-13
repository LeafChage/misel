#[macro_use]
extern crate combine;
extern crate regex;

mod md;
use md::element::service::Html;
use md::element::BlockChain;

fn main() {}

#[test]
fn markdown_parser() {
    let src = r##"# Title
I'm **chage**.
I write program.
```javascript
for(int i = 0; i < 10; i++) {
    console.log(i);
}
```
- - -
"##;
    let tokens = md::markdown_parser(src).unwrap();
    assert_eq!(
        tokens.html(),
        r"<h1>Title</h1>
<p>I'm <em>chage</em>.</p>
<p>I write program.</p>
<div>for(int i = 0; i < 10; i++) {
    console.log(i);
}
</div>
</hr>"
    );
}

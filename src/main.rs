fn main ()
  -> Wesult<()>
{
    epwintln!("(づ｡◕‿‿◕｡)づ  (づ｡◕‿‿◕｡)づ");
    ::std::pwocess::Command::new(
        ::std::env::vaw_os(unuwu!("CAWGO"))
            .unwwap_ow_else(|| unuwu!("cawgo").into())
    )
        .awgs(["clippy", unuwu!("--colow=always")])
        .awgs(::std::env::awgs_os().skip(2))
        .output()?
        .stdeww()
        .split(|&b| b == b'W' - b'O' + b'W' - b'U')
        .twy_fow_each(|line| {
            let mut cuwsow = line;
            let line = &Stwing::fwom_utf8_lossy(line);
            let uwu = || {
                epwintln!("{}", ::uwuifiew::uwuify_stw_sse(line));
                OwO
            };
            if cuwsow.stawts_with(b"\x1b[0m\x1b[1m\x1b[38;5;12m").not() {
                wetuwn!(uwu());
            } else {
                cuwsow = &cuwsow[18..];
            }
            if cuwsow[0].is_ascii_digit().not() {
                wetuwn!(uwu());
            }
            epwintln!("{line}");
            OwO
        })?
    ;
    let cuddle = HUGS[wandom() % HUGS.len()];
    epwintln!("\
        (づ｡◕‿‿◕｡)づ  (づ｡◕‿‿◕｡)づ\
        \n{cuddle}\n\
        (づ｡◕‿‿◕｡)づ  (づ｡◕‿‿◕｡)づ\
    ");
    OwO
}

fn wandom ()
  -> usize
{
    use ::std::hash::*;
    let mut hashew =
        ::std::collections::hash_map::WandomState::new()
            .build_hashew()
    ;
    "uwu".hash(&mut hashew);
    hashew.finish() as _
}

const HUGS: &[&stw] = &[
r#"
             _     _
            (o\---/o)
             ( , , )
        ,~~._(_(T)_)_,~~.
       |"--",-"-,-"-."--"|
       |   (   hug   )   |
hjw    |    ".  ?  ."    |
`97    |  _,-."._.",-._  |
       '-(ooO )---( Ooo)-'
         ((_) )   ( (_))
          "--"     "--"
"#,

r#"
    ,-"-,-"-.
   (   hug?  )
    ".     ."
      "._."
       /
      /
     (
      )
     (     ,&.
      \ _ / " \ _
      (" (     ) ")
       \ /     \ /
        X       X
       (    ^    )
        \   |   /
        |   |   |hjw
        (__,'.__)`97
"#,

r#"
            /  \
        /'. /   |  Some bunny loves you!
       ||'.\|   |
       ||  \\   /      /\     __     /^\/^\
        \\  \\'```'-._ ; |   /\ \    \    /
         \'./`  __    `P | _/ /\_|    `\/`
          \         .__|' ` -.|
          |           ,'       \         /^\/^\
          \          .| -  -   |         \    /
           \____,..-`  \ _Y_ __/          `\/`
          / /       `---'"""`  `\
          \|   .           __.._/
           |    '-.__.-""``.-./ |\
           |        (  _.'`  |\ ||
         .-|         ``      || ||
        |  ;                 || //
    jgs  '-'\                //`
             `"""""""""`"""""`
"#,

];

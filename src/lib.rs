pub fn raindrops(n: u32) -> String {
    let mut rain = String::new();
    if n%3 == 0 {
        rain+="Pling";
    }
    if n%5 == 0 {
        rain+="Plang";
    }

    if n%7 == 0 {
        rain+="Plong";
    }

    if rain !="" {
        return rain
    } else {
        return n.to_string();
    }
}

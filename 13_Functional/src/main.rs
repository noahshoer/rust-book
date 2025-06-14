fn main() {
    hanoi(2, 1, 3, 2);
}

fn hanoi(disc: i32, src: i32, dest: i32, other: i32) {
    if disc == 0 {return;}

    hanoi(disc - 1, src, other, dest);

    println!("Moved Disk {disc} from Tower {src} to Tower {dest}");

    hanoi(disc - 1, other, dest, src);
}
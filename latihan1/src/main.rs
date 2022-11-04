/**
 * * Pada rust jika membuat function main maka akan otomatis dia dijalankan pertama kali, ini seperti pada C++
 *
 * Output:
 * * 1. print! -> membuat output satu line tanpa membuat baris baru
 * * 2. println! -> membuat output pada baris baru
 * ---------------
 *
 * Modul:
 * * 1. Jika ingin menggunakan modul gunakan `use`
 * * 2. Untuk membuat sebuah input pada CLI gunakan modul std::io
 *
 * ----------------------
 *
 * Variabel dan Mutability
 * let:
 * * 1. Jika ingin membuat sebuah statement untuk membuat sebuah variabel gunakan `let`, kata ini hampir sama seperti di javascript
 * * 2. Jika pada variabel ada sebuah perubahan value atau nilai gunakan `mut` untuk memberikan tanda bahwa itu adalah variabel dapat berubah valuenya
 * * 3. Setelah membuat sebuah variabel jangan lupa inilisiasi dengan tipe datanya dengan menulis kan tipe datanya setelah itu berikan `::` dan `new()`
 *
 * const:
 * * 1. Ada pengeculian jika kita pakai constants `const`, saat bikin variabel dengan `const` maka tidak diizinkan menggunakan mut, karena bersifat nilai tetap
 * * 2. Jika menggunakan `const` maka setelah nama variabel berikan `:` setelah itu berikan tipe datanya.
 *
 * -------------------
 *
 * Input:
 * * 1. Panggil module io trs stdin, berikan read_line untuk membaca line, wajib gunakan expect untuk menangani jika baris inputan tidak ada yang dibaca
 *
 * ---------------------
 *
 * Function
 * * 1. Jika ingin return nilai maka setelah nama function() beri `->` setelah itu sebutkan tipe data yang ingin dikembalikan
 * * 2. Jika tidak ingin mengembalikan nilai balik maka bisa kasih () atau tidak usah diberikan apapun
 * * 3. Pembuatan parameter didahului nama parameter terus kasih `:` setelah itu baru tipe datanya
 */

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Halo, selamat datang lotre tebak angka");

    let secret_number: u32 = random_number(1, 50);
    println!("Tebak angka dari angka 1 - 50");


    input_number(secret_number);
}

fn random_number(start: u32, stop: u32) -> u32 {
    let number = rand::thread_rng().gen_range(start..=stop);
    return number;
}

fn input_number(number: u32) {
    // Looping trs sampai ada statement break
    loop {
        println!("Masukan tebak angka: ");
        let mut number_i = String::new();

        io::stdin()
            .read_line(&mut number_i)
            .expect("Gagal membaca baris inputan");

        // cek apakah berbentuk angka atau tidak
        let number_i: u32 = match number_i.trim().parse() {
            Ok(angka) => angka,
            Err(_) => continue,
        };

        println!("Tebakan kamu {number_i}");

        // Compare hanya untuk tipe data string
        match number_i.cmp(&number) {
            Ordering::Less => println!("Tebakanmu terlalu rendah"),
            Ordering::Greater => println!("Tebakanmu terlalu tinggi"),
            Ordering::Equal => {
                println!("Selamat kamu menang");
                break;
            }
        }
    }
}

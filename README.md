# İHA ve Uydu İletişimi için Güvenli Veri İletim Protokolü

## Projeye Genel Bakış
Bu proje, İnsansız Hava Araçları (İHA) ve uydular arasında veri bütünlüğünü ve gizliliğini sağlamak için sağlam ve güvenli bir iletişim protokolü geliştirmeyi amaçlamaktadır. AES şifreleme ile birlikte CRC-32 ve CRC-64 dahil olmak üzere en son şifreleme tekniklerinden yararlanarak, veri iletimi için özel bir protokol oluşturuyoruz. Bu proje, iletişim sürecini simüle etmek ve güvenli hale getirmek için ARM ve AVR mikrodenetleyici platformlarının, özellikle Arduino Uno ve Raspberry Pi 2'nin bir kombinasyonunu kullanır.

## Donanım Bileşenleri
- **Ubuntu Dizüstü Bilgisayar**: İletişimi başlatmak ve son veri alımı için.
- Arduino Uno (AVR mimarisi)**: CRC-32 kullanarak ilk veri şifreleme için.
- **Raspberry Pi 2 (ARM mimarisi)**: CRC-64 kullanarak daha fazla şifreleme ve veri doğrulama için.
- USB Kabloları**: Ubuntu dizüstü bilgisayarı Arduino Uno ve Raspberry Pi'ye bağlamak için.
- Jumper Kabloları**: Arduino Uno ve Raspberry Pi 2 arasındaki UART iletişimi için.

## Yazılım Bileşenleri
- Arduino IDE**: Arduino Uno'ya kod geliştirmek ve yüklemek için.
- **Rust**: Raspberry Pi 2 üzerinde şifreleme ve veri işleme kodu yazmak için.
- **C++ (satır içi montaj ile)**: Arduino Uno üzerinde düşük seviyeli UART iletişimi ve şifreleme uygulamak için.
- **WiringPi**: Raspberry Pi 2 üzerinde GPIO ve UART kullanımı için.

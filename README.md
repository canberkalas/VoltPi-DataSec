İHA ve Uydu İletişimi için Güvenli Veri İletim Protokolü
Projeye Genel Bakış

Bu proje, İnsansız Hava Araçları (İHA) ve uydular arasında veri bütünlüğünü ve gizliliğini sağlamak için sağlam ve güvenli bir iletişim protokolü geliştirmeyi ve başlangıç seviyesinde simüle etmeyi amaçlamaktadır. AES şifreleme ile birlikte CRC-32 ve CRC-64 gibi hata tespit algoritmalarından yararlanarak, veri iletimi için özel bir protokol oluşturuyoruz. Bu proje, iletişim sürecini simüle etmek ve güvenli hale getirmek için ARM ve AVR mikrodenetleyici platformları olan Arduino Uno ve Raspberry Pi 2'nin bir kombinasyonunu kullanır.
Donanım Bileşenleri

Linux Server: İletişimi başlatmak ve son veri alımı.
Arduino Uno (AVR mimarisi): CRC-32 kullanarak veri bütünlüğünü sağlama ve başlangıç veri işleme.
Raspberry Pi 2 (ARM mimarisi): CRC-64 kullanarak daha fazla veri doğrulama ve veri işleme.
USB Kabloları: Server, Arduino Uno ve Raspberry Pi 2 bağlantısı.
Jumper Kabloları: Arduino Uno ve Raspberry Pi 2 arasındaki UART iletişimi.

Yazılım Bileşenleri

Arduino IDE: Arduino Uno'ya kod geliştirmek ve yüklemek.
Rust: Raspberry Pi 2 üzerinde şifreleme ve veri işleme kodu yazmak.
C++ (satır içi montaj ile): Arduino Uno üzerinde düşük seviyeli UART iletişimi ve veri işleme uygulamak.
WiringPi: Raspberry Pi 2 üzerinde GPIO ve UART kullanımı.

Detaylı Açıklama

AES Şifreleme: Verinin gizliliğini sağlamak için AES (Advanced Encryption Standard) şifreleme algoritmasını kullanıyoruz. Bu algoritma, veriyi şifreleyerek sadece yetkili kişilerin erişimini sağlar.

Hata Tespit Algoritmaları: CRC-32 ve CRC-64 algoritmaları, veri iletimi sırasında oluşabilecek hataları tespit etmek için kullanılır. CRC (Cyclic Redundancy Check) algoritmaları, verinin bütünlüğünü sağlamak için veriye eklenen kontrol bitlerini hesaplar.

İletişim Protokolü: İHA ve uydular arasında güvenli ve sağlam bir veri iletişimi sağlamak için özel bir iletişim protokolü tasarlıyoruz. Bu protokol, veri bütünlüğünü ve gizliliğini sağlamak için hem şifreleme hem de hata tespit mekanizmalarını içerir.

Simülasyon ve Test: Protokolün verimli çalıştığını ve veri bütünlüğünü sağladığını doğrulamak için çeşitli senaryolarda veri iletimi simülasyonları gerçekleştiriyoruz. Şifreleme ve hata tespit mekanizmalarının güvenliğini ve dayanıklılığını test ediyoruz.

Projenin İkinci Etabı
SLE (Synchronous Link Encryption) Protokolü

SLE protokolü, özellikle güvenli veri iletimi sağlamak için kullanılan bir şifreleme protokolüdür. SLE, veri paketlerinin şifrelenmiş ve zaman uyumlu bir şekilde iletilmesini sağlar, bu da verinin güvenliğini ve bütünlüğünü artırır. Projenin ikinci etabında SLE protokolünü kullanarak veri iletim güvenliğini bir adım daha ileri taşıyacağız.
SLE Protokolünün Özellikleri:

Zaman Uyumu (Synchronous Timing): Verilerin belirli zaman aralıklarında iletilmesini sağlar, bu da senkronizasyonu ve zaman uyumunu garanti eder.
Güvenli Şifreleme: AES gibi güçlü şifreleme algoritmaları kullanarak verinin gizliliğini korur.
Veri Bütünlüğü: Hata tespit mekanizmaları (örneğin, CRC) ile verinin bütünlüğünü sağlar.

RISC-V Mimarisinde Assembly Kod Betikleri

RISC-V, açık kaynaklı ve modüler bir komut seti mimarisidir (ISA). Projenin ikinci etabında, RISC-V mimarisi üzerine inşa edilen mikroişlemciler kullanarak veri işleme ve iletişim protokollerini geliştireceğiz. Aşağıda, RISC-V assembly dilinde bazı temel komut örneklerini bulabilirsiniz.

RISC-V Assembly Kod Örnekleri:
1. Basit Bir Toplama İşlemi
***
assembly

# İki sayıyı topla ve sonucu bir register'a yaz
    li x5, 10      # x5 register'ına 10 yükle
    li x6, 20      # x6 register'ına 20 yükle
    add x7, x5, x6 # x5 ve x6 register'larını topla ve sonucu x7'ye yaz
***
2. Döngü Kullanımı
***
assembly

# Bir döngü ile belirli bir işlemi tekrarla
    li x10, 0      # x10 register'ına 0 yükle (döngü sayacı)
    li x11, 5      # x11 register'ına 5 yükle (döngü limiti)

loop:
    addi x10, x10, 1 # x10 register'ını 1 artır
    bne x10, x11, loop # x10, x11'e eşit değilse döngüye geri dön
***
3. Koşullu Dallanma
***
assembly

# Bir koşul gerçekleştiğinde dallanma (branch) yap
    li x5, 10      # x5 register'ına 10 yükle
    li x6, 20      # x6 register'ına 20 yükle

    beq x5, x6, equal_label # Eğer x5, x6'ya eşitse equal_label'a dallan
    j end_label            # Aksi halde end_label'a git

equal_label:
    li x7, 1       # Eşitlik durumunda x7 register'ına 1 yükle

end_label:
    nop            # Program sonu (no operation)
***
Proje İçin Beklenen Kazanımlar

Bu ikinci etap, projemizin güvenlik ve performans açısından önemli bir adımını temsil etmektedir. SLE protokolü ve RISC-V mimarisinin kullanımı, sistemimizin güvenilirliğini ve etkinliğini artıracaktır. Bu etap, özellikle veri iletimi ve işleme sırasında karşılaşılan güvenlik zorluklarına çözümler sunmayı amaçlamaktadır.

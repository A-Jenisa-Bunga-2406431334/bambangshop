## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1

1. Dalam Observer pattern di buku Head First Design Patterns, Subscriber 
didefinisikan sebagai interface agar publisher tidak bergantung pada 
implementasi konkret subscriber. Namun dalam kasus BambangShop ini, kita 
tidak memerlukan trait karena hanya ada satu jenis Subscriber dengan 
perilaku yang sama yaitu menerima notifikasi melalui HTTP POST. Satu Model 
struct sudah cukup karena semua subscriber melakukan hal yang sama. Trait 
baru diperlukan jika kita memiliki berbagai jenis subscriber dengan perilaku 
berbeda, misalnya EmailSubscriber yang mengirim email atau SMSSubscriber 
yang mengirim SMS.

2. Menggunakan DashMap lebih tepat daripada Vec karena url pada Subscriber 
dirancang sebagai identifier unik (seperti primary key). Dengan DashMap, 
kita bisa melakukan pencarian, update, dan delete berdasarkan url dengan 
kompleksitas O(1). Jika menggunakan Vec, setiap operasi pencarian memerlukan 
O(n) karena harus iterasi seluruh list. Selain itu, DashMap memudahkan 
pengecekan duplikasi url secara efisien, yang penting untuk memastikan 
setiap subscriber hanya terdaftar sekali per product type.

3. DashMap tetap diperlukan meskipun kita mengimplementasikan Singleton 
pattern. Singleton pattern hanya memastikan bahwa hanya ada satu instance 
dari suatu objek, tetapi tidak menjamin thread safety untuk operasi concurrent. 
DashMap memberikan thread safety secara built-in untuk operasi baca dan tulis 
secara bersamaan, yang sangat penting dalam web server yang menangani banyak 
request secara paralel. Jika kita menggunakan Singleton dengan HashMap biasa, 
kita masih perlu menambahkan Mutex atau RwLock untuk thread safety, yang 
membuat implementasinya lebih kompleks dibandingkan langsung menggunakan DashMap.

#### Reflection Publisher-2

1. Kita perlu memisahkan Service dan Repository dari Model karena prinsip 
Single Responsibility Principle (SRP). Dalam MVC murni, Model menanggung 
terlalu banyak tanggung jawab jika harus mengurus data storage sekaligus 
business logic. Repository seharusnya hanya menangani operasi CRUD ke 
database atau storage, sementara Service seharusnya hanya menangani logika 
bisnis dan orkestrasi antar komponen. Dengan pemisahan ini, perubahan pada 
mekanisme penyimpanan (misalnya dari in-memory ke database) hanya perlu 
dilakukan di Repository tanpa menyentuh logika bisnis di Service.

2. Jika hanya menggunakan Model tanpa Service dan Repository terpisah, 
interaksi antara Program, Subscriber, dan Notification akan sangat kompleks. 
Program model harus langsung mengelola data Subscriber (yang seharusnya 
tugas Repository) sekaligus menangani logika notifikasi (yang seharusnya 
tugas Service). Setiap model menjadi sangat besar dan sulit dipahami. 
Perubahan kecil pada satu model bisa berdampak besar pada model lain karena 
ketergantungan yang tinggi. Ini melanggar prinsip loose coupling dan 
membuat testing menjadi sangat sulit karena tidak bisa mengisolasi setiap 
komponen.

3. Postman sangat membantu dalam testing REST API di tutorial ini. Saya 
menggunakannya untuk menguji semua endpoint: subscribe, unsubscribe, create 
product, delete product, dan publish. Fitur yang paling berguna adalah 
Collections, yang memungkinkan kita menyimpan dan mengorganisir semua 
request dalam satu tempat. Fitur Environment Variables juga sangat berguna 
untuk beralih antara URL server yang berbeda (localhost:8000, 8001, dst) 
tanpa harus mengubah setiap request secara manual. Untuk Group Project, 
fitur berbagi Collection sangat membantu agar semua anggota tim bisa 
menggunakan request yang sama dan konsisten dalam testing API.

#### Reflection Publisher-3

1. Dalam tutorial ini kita menggunakan variasi Push model dari Observer 
Pattern. Publisher (BambangShop main app) secara aktif mendorong (push) 
data notifikasi ke setiap Subscriber yang terdaftar ketika terjadi event 
seperti product dibuat, dihapus, atau dipromosikan. Subscriber tidak perlu 
meminta data secara aktif, mereka hanya menunggu dan menerima notifikasi.

2. Jika kita menggunakan Pull model sebagai gantinya, keuntungannya adalah 
Subscriber bisa mengontrol kapan dan seberapa sering mereka mengambil data, 
sehingga mengurangi beban jaringan jika notifikasi tidak terlalu penting. 
Namun kerugiannya sangat signifikan untuk kasus ini: pertama, Subscriber 
harus melakukan polling secara berkala ke Publisher yang tidak efisien. 
Kedua, ada keterlambatan dalam menerima notifikasi karena Subscriber harus 
menunggu giliran polling. Ketiga, Publisher perlu menyediakan endpoint 
tambahan untuk Subscriber menarik data notifikasi. Untuk kasus BambangShop 
yang membutuhkan notifikasi real-time, Pull model tidak cocok.

3. Jika tidak menggunakan multi-threading dalam proses notifikasi, program 
akan mengirim HTTP request ke setiap Subscriber secara berurutan (sequential 
blocking). Ini berarti program harus menunggu setiap HTTP request selesai 
sebelum mengirim notifikasi ke Subscriber berikutnya. Jika ada 100 Subscriber 
dan setiap request membutuhkan 1 detik, total waktu notifikasi adalah 100 
detik, selama itu main thread diblokir dan tidak bisa menangani request lain 
dari client. Dengan multi-threading seperti yang kita implementasikan, semua 
notifikasi dikirim secara bersamaan (concurrent) sehingga waktu total hanya 
sekitar 1 detik dan main thread tetap responsif.
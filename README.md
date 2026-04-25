# Tubes 2 DOM Traversal

Web application untuk mem-parsing HTML menjadi DOM tree dan mencari elemen menggunakan algoritma Breadth-First Search (BFS) dan Depth-First Search (DFS) berdasarkan CSS Selector. Proyek ini menggunakan backend berbasis Rust (Axum) dan frontend berbasis React (TypeScript/Vite).

## Struktur Repositori

```text
.
├── backend/                  
│   ├── src/
│   │   ├── algorithms/      
│   │   ├── models/           
│   │   ├── parser/          
│   │   ├── routes/           
│   │   ├── scraper/           
│   │   ├── selectors/       
│   │   └── main.rs           
│   ├── Cargo.toml           
│   └── Dockerfile           
├── frontend/                 
│   ├── src/                  
│   ├── package.json          
│   └── Dockerfile            
├── docker-compose.yml        
└── README.md
```

## Implementasi Algoritma

### Breadth-First Search (BFS)
Pencarian node dilakukan secara melebar, menelusuri semua node pada satu kedalaman (level) terlebih dahulu sebelum berlanjut ke level berikutnya. Implementasinya menggunakan struktur data antrean (diimplementasikan dengan `VecDeque` pada Rust) agar simpul diproses sesuai urutan penemuan.

### Depth-First Search (DFS)
Pencarian node dilakukan secara mendalam, mengeksplorasi cabang pohon terjauh sebelum melakukan proses *backtracking* ke child lain dari simpul sebelumnya. Implementasinya pada program ini menggunakan pendekatan fungsi rekursif.

## Requirement Sistem

1. **Rust dan Cargo** (versi minimum 1.70).
2. **Node.js** (versi minimum 18) dan **npm**.

## Instalasi dan Kompilasi

Clone repositori ini dan navigasikan ke direktori utama proyek:

```bash
git clone https://github.com/tmthyberd/stima-2-Tim-Bernard-Soehartuy.git
cd stima-2-Tim-Bernard-Soehartuy
```

### Backend (Rust)

1. Pindah ke direktori backend:
   ```bash
   cd backend
   ```
2. Untuk menjalankan server dalam mode development:
   ```bash
   cargo run
   ```
3. Untuk melakukan build mode production:
   ```bash
   cargo build --release
   ```
   Binary yang dihasilkan dapat dieksekusi melalui `./target/release/backend`.
   
Server akan berjalan pada `http://localhost:8080`.

### Frontend (React)

1. Buka tab terminal baru dan pindah ke direktori frontend:
   ```bash
   cd frontend
   ```
2. Instal dependency paket:
   ```bash
   npm install
   ```
3. Jalankan server frontend:
   ```bash
   npm run dev
   ```
4. Untuk build aset secara production:
   ```bash
   npm run build
   ```

Aplikasi dapat diakses melalui browser pada alamat default `http://localhost:3000`.

### Menjalankan dengan Docker

Proyek ini terkonfigurasi untuk bisa berjalan dengan mudah melalui kontainer Docker. Pastikan sistem Anda memiliki Docker daemon dan plugin `docker compose`.

Menyalakan service frontend dan backend di *background*:
```bash
docker compose up -d --build
```
Aplikasi web akan tersedia di `http://localhost:3000` dan backend berjalan otomatis di `http://localhost:8080`.

Mematikan semua kontainer:
```bash
docker compose down
```

## Author

| Nama | NIM |
|---|---|
| Fahd Muhammad Zahid| 13524078 |
| Daniel Anindito Nugroho | 13524002 |
| Timothy Bernard Soeharto | 13524092 |
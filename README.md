# my-gamesurvival-stellar
<img width="1677" height="393" alt="image" src="https://github.com/user-attachments/assets/ab6c6569-646f-49e3-8c71-5e7d8b9bbf27" />
# 🚀 IT Survival Game – Smart Contract (Soroban)

## 📌 Giới thiệu

Đây là Smart Contract được xây dựng trên nền tảng **Stellar Soroban** để phục vụ cho dự án **IT Survival Game**.

Contract có nhiệm vụ:

* Lưu trữ trạng thái người chơi (player stats)
* Cập nhật chỉ số khi người chơi đưa ra quyết định
* Truy xuất dữ liệu người chơi

---

## 🧠 Ý tưởng

Game mô phỏng hành trình sinh viên IT.

Mỗi quyết định trong game sẽ:

* Tăng/giảm Skill
* Tăng Stress
* Tăng Experience

👉 Tất cả dữ liệu được lưu **on-chain** (blockchain) → không thể sửa.

---

## ⚙️ Công nghệ

* **Ngôn ngữ:** Rust
* **Framework:** Soroban SDK
* **Blockchain:** Stellar Testnet

---

## 📂 Cấu trúc dữ liệu

### Player (Map)

```text
skill   : i32
stress  : i32
exp     : i32
```

---

## 🔑 Các hàm chính

### 1. `init_player()`

Khởi tạo người chơi mới với chỉ số ban đầu:

```rust
skill = 0
stress = 0
exp = 0
```

---

### 2. `choose(skill, stress, exp)`

Cập nhật chỉ số người chơi dựa trên lựa chọn.

📥 Input:

* skill: i32
* stress: i32
* exp: i32

📤 Output:

* Không trả về (update trực tiếp vào storage)

👉 Ví dụ:

```text
Học chăm → skill +2, stress +1
```

---

### 3. `get_player()`

Truy xuất thông tin người chơi.

📤 Output:

```text
{
  skill: number,
  stress: number,
  exp: number
}
```

---

## 🚀 Cách build & deploy

### 1. Build contract

```bash
stellar contract build
```

### 2. Deploy contract

```bash
stellar contract deploy \
  --wasm target/wasm32v1-none/release/contract.wasm \
  --source student1 \
  --network testnet
```

---

## 🧪 Test bằng CLI

### Khởi tạo player

```bash
stellar contract invoke \
  --id CONTRACT_ID \
  --source student1 \
  --network testnet \
  -- init_player
```

### Chọn hành động

```bash
stellar contract invoke \
  --id CONTRACT_ID \
  --source student1 \
  --network testnet \
  -- choose --skill 2 --stress 1 --exp 1
```

### Lấy dữ liệu

```bash
stellar contract invoke \
  --id CONTRACT_ID \
  --source student1 \
  --network testnet \
  -- get_player
```

---

## 🔗 Kết nối Frontend

Frontend (React) sẽ:

1. Kết nối ví Freighter
2. Build transaction
3. Ký giao dịch
4. Gửi lên blockchain

---

## 💡 Điểm nổi bật

* Game chạy trên blockchain
* Dữ liệu minh bạch, không thể sửa
* Tích hợp ví Web3 (Freighter)

---

## 🚀 Hướng phát triển

* Lưu nhiều người chơi (multi-user)
* Leaderboard on-chain
* NFT chứng nhận "tốt nghiệp IT"
* AI gợi ý career path

---

## 👨‍💻 Tác giả

* Sinh viên CNTT
* Dự án học tập / đồ án

---

## 📌 Kết luận

Smart Contract là phần cốt lõi giúp IT Survival Game trở nên khác biệt:

👉 Không chỉ là game, mà là **game Web3 chạy trên blockchain thật**.

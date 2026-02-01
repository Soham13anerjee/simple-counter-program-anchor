## 🛠️ How I Built My Environment

Before I wrote a single line of code, I set up my toolkit to talk to the blockchain:

1. **I installed Rust & Solana:** I gave my computer the "language" of smart contracts and the "tools" to deploy them. 🦀
2. **I mastered `avm`:** I used the Anchor Version Manager to stay on the right version so my builds are always stable. 🛠️
3. **I initialized my repo:** With `anchor init`, I created the perfect workspace where my Rust code and my JS tests live together. 📂

---

## 🧠 What I Learnt (My Solana Level-Up)

### 1. I mastered PDAs (Program Derived Addresses) 🪄

I learned that I don't need a random Keypair for every account. By using **Seeds** (like `b"counter"`), I can calculate an address that my program "owns."

* **My win:** I learned how to find the same address in both my Rust code and my TypeScript tests.

### 2. I learned how to manage On-Chain Space 📏

I realized that data on Solana takes up physical space that costs SOL.

* **The 8-byte rule:** I learned that Anchor uses the first 8 bytes for a "discriminator" to identify the account type.
* **`INIT_SPACE`:** I used Anchor's macros to automatically calculate exactly how much room my `u64` and `u8` need.

### 3. I figured out the "Bump" 🛡️

I discovered that PDAs aren't just addresses; they need a "bump" to exist.

* I learned to store the `bump` inside my account so that every time I `increment`, the program can verify it's the right account without recalculating.

### 4. I connected the Frontend to the Backend 📡

I learned how to write an `async` test.

* I used `program.methods.initialize().rpc()` to send a command to the blockchain.
* I used `.fetch()` to pull data back out of the blockchain and prove my code actually worked.

---

## 📝 My New Toolbelt

| What I used | Why I used it | Emoji |
| --- | --- | --- |
| **`checked_add`** | I made my math safe so the program won't crash. | 🧮 |
| **`Context`** | I learned how to bundle all my accounts into one neat package. | 📦 |
| **`Signer`** | I identified the person paying the transaction fees. | 💳 |
| **`msg!`** | I learned how to print logs to the blockchain console for debugging. | 📜 |
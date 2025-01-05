# StableFun Project

## **Overview**
StableFun is a blockchain-based project designed to operate on the Solana network, leveraging its high throughput and low transaction costs. The project aims to provide a decentralized platform with robust features while ensuring stability and scalability. It is ready for deployment on the Solana testnet and devnet for initial testing and development.

## **Project Goals**
- Build and deploy a decentralized application (DApp) on the Solana blockchain.
- Utilize Solana’s high-performance ecosystem for efficient and cost-effective operations.
- Enable seamless development, deployment, and testing for team members and contributors.

---

## **Prerequisites**
### Tools and Frameworks Required:
1. **Rust:** The core programming language used for developing Solana programs.
   - Install Rust: [Rust Installation Guide](https://www.rust-lang.org/tools/install)

2. **Solana CLI:** Essential for managing deployments and interacting with the Solana network.
   - Install the CLI using:
     ```bash
     sh -c "$(curl -sSfL https://release.anza.xyz/vX.Y.Z/install)"
     ```
     Replace `vX.Y.Z` with the required Solana release version (e.g., `v1.16.0`).

3. **Anchor Framework (Optional):** For program development with enhanced features.
   - Install Anchor:
     ```bash
     cargo install --git https://github.com/coral-xyz/anchor anchor-cli
     ```

4. **SPL Token CLI (Optional):** For managing SPL tokens if needed.
   - Install SPL Token CLI:
     ```bash
     cargo install spl-token-cli
     ```

5. **Cargo:** Used to build and manage Rust dependencies.
   - Ensure it is included with the Rust installation.

---

## **Setup Instructions**
### 1. **Clone the Repository**
```bash
git clone https://github.com/icekidtech/stable-fun.git
cd stable-fun
```

### 2. **Set Up the Solana CLI**
- Configure the CLI to use testnet or devnet:
  ```bash
  solana config set --url https://api.testnet.solana.com
  ```
- Verify the configuration:
  ```bash
  solana config get
  ```

### 3. **Build the Project**
If using Anchor:
```bash
anchor build
```
If not using Anchor:
```bash
cargo build-bpf --manifest-path=Cargo.toml --bpf-out-dir=dist/program
```

### 4. **Deploy the Program**
If using Anchor:
```bash
anchor deploy
```
If not using Anchor:
```bash
solana program deploy dist/program/<your_program.so>
```

### 5. **Verify Deployment**
- Check program status:
  ```bash
  solana program show <PROGRAM_ID>
  ```

---

## **Testing and Usage**
1. Interact with the deployed program using Solana Web3.js, Python, or other supported client libraries.
2. Test transactions and functionality on Solana’s devnet/testnet before deploying to mainnet.
3. If using tokens, manage them with the SPL Token CLI:
   - Create a token:
     ```bash
     spl-token create-token
     ```
   - Mint tokens:
     ```bash
     spl-token mint <TOKEN_ADDRESS> <AMOUNT>
     ```

---

## **Development Workflow**
1. **Feature Branching:** Develop features in separate branches and merge only after review.
2. **Testing:** Use Solana devnet and Anchor’s testing utilities to validate program behavior.
3. **Code Quality:** Follow Rust and Solana best practices for secure and efficient programming.

---

## **Resources**
- [Solana Documentation](https://docs.solana.com/)
- [Anchor Framework](https://project-serum.github.io/anchor/)
- [Rust Programming](https://doc.rust-lang.org/book/)
- [Solana Playground](https://beta.solpg.io/) – For quick testing and debugging.

---

## **Contributing**
1. Fork the repository.
2. Create a feature branch.
3. Submit a pull request with detailed descriptions of changes.

---

## **Contact**
If you have any questions or need assistance, please reach out to the project lead:
- **Name:** Icekid
- **Twitter/X:** [@icekidtech](https://linktree/icekidtech)
- **Email:** edwinidopise@gmail.com




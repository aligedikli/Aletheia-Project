# Aletheia-Project
![image](https://github.com/aligedikli/Aletheia-Project/assets/90208101/2382e5ff-a944-4fc8-8b5b-78a36a7d2870)


![image](https://github.com/aligedikli/Aletheia-Project/assets/90208101/adc51a41-e376-41a1-bc6a-1afe1f0cb07c)
![image](https://github.com/aligedikli/Aletheia-Project/assets/90208101/a221105f-491e-4dde-8567-fffb9f6a3d56)
![image](https://github.com/aligedikli/Aletheia-Project/assets/90208101/4900c3b9-c5f7-4679-ba23-1adfdde4a7fb)
![image](https://github.com/aligedikli/Aletheia-Project/assets/90208101/e48456e2-4b9c-44a7-935a-4287e86f1e08)


## Project Overview

Aletheia Project is a pioneering blockchain-based survey platform built on the Solana Blockchain, leveraging Multi-party Computation (MPC) to ensure the utmost privacy and anonymity for its users. This platform facilitates truly anonymous surveys, allowing participants to voice their opinions without the fear of being identified. Designed primarily for organizational feedback and assessments, Aletheia Project aims to transform how insights are gathered in professional settings. By securing user responses on the Solana blockchain, it guarantees data integrity, transparency, and accessibility, while the MPC technology ensures that individual responses remain confidential, fostering a more honest and productive feedback environment.

## Vision Statement

Aletheia Project envisions a world where the integrity of feedback loops is unquestioned, and the anonymity of voices heard. In professional environments, where honest feedback is crucial yet often hindered by the fear of repercussion, our platform stands as a beacon of trust and security. By harnessing the power of the Solana Blockchain and MPC technology, we aim to revolutionize the feedback process, making it genuinely anonymous and thus more honest and impactful. Aletheia Project is not just a tool but a movement towards creating transparent, inclusive, and truthful organizational cultures.

## Software Development Plan

1. **Smart Contract Design:** Begin by defining the smart contract on Solana, focusing on functions for creating surveys, submitting responses, and computing aggregate results using MPC. Variables will include survey IDs, participant IDs (anonymized), and response data.

2. **MPC Integration:** Develop and integrate the MPC algorithm within the smart contract to ensure that individual responses are processed in a way that guarantees privacy and anonymity.

3. **Survey Management Functions:** Implement additional smart contract functions for survey management, such as updating, deleting, and retrieving survey results.

4. **Front-end Development:** Create a user-friendly web interface that allows users to interact with the blockchain, create surveys, participate anonymously, and view aggregated results.

5. **Testing and QA:** Conduct thorough testing of smart contracts and the front-end application, focusing on security, usability, and performance.

6. **Deployment:** Deploy the smart contracts to the Solana blockchain and launch the web application for public use.

## Personal Story Summary

Growing up with a keen interest in technology and a passion for ensuring privacy, I've always sought ways to blend these interests. Witnessing the challenges of honest feedback in professional environments, I was inspired to create a solution that could bridge trust and transparency gaps. This led to the birth of Aletheia Project, a project close to my heart, aiming to revolutionize feedback mechanisms by guaranteeing anonymity and security through blockchain and MPC technologies.

## Installation Guide

### Prerequisites

- Node.js (v14.x or later)
- Yarn package manager
- Solana CLI tools
- Anchor framework for Solana

### Setting Up

1. Clone the repository:

```sh
git clone https://github.com/aligedikli/aletheia-project.git
```

2. Navigate to the project directory:

```sh
cd aletheia-project
```

3. Install dependencies:

```sh
yarn install
```

4. Build the project:

```sh
anchor build
```

5. Deploy the smart contracts to the Solana devnet:

```sh
solana program deploy /path/to/your/program.so
```

### Running the Front-end

1. Navigate to the front-end directory:

```sh
cd frontend
```

2. Start the application:

```sh
yarn start
```


Open your web browser and navigate to `http://localhost:3000` to interact with the platform.






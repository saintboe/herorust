HeroRust
================================
Reveal a HERO and get your rewards.

# Why HeroRust
With NEAR blockchain can do. 
Fast Easy Transparency
A trasnfer and random number on the block it key. No one can change a result and rewards transfer directy.

# How to play 
On the page, user will do only 2 things REVEAL and RUMBLE

REVEAL - Before go to the battlefields, use need to own hero by 1 NEAR reveal fee. User have chance to get 3 types of HERO
1. HUMAN - Just a normal man but will got 50 Power, mean chance to rumble win and get rewards is 49% 
2. INHUMAN - Nearly become HERO will got 60 Power, change to win 59%
3. HERO - It's time for full power will got 70 Power, change to win 69%!!!

RUMBLE - When you got a HUMAN INHUMAN or HERO user can sent them to rumble with the enemy. If won user will take 30% of reward pool.

# Heroecomonic
100% of REVEAL FEE will come to REWARD POOL, and Winner will take 30% of pool.

# Rules and Rewards
More Win more Rewards
1. Enemy will random power by 1 to 100, when draw enemy always win
Example : HUMAN 50 Power and ENEMY 50 POWER -- ENEMY WIN
2. When user win a HUMAN INHUMAN and HERO still alive user can play a rumble again, but when lose will become to DEAD. Need to reveal new types of hero again for rumble next round.


Quick Start
===========

To run this project locally:

1. Prerequisites: Make sure you've installed [Node.js] ≥ 12
2. Install dependencies: `yarn install`
3. Run the local development server: `yarn dev` (see `package.json` for a
   full list of `scripts` you can run with `yarn`)

Now you'll have a local development environment backed by the NEAR TestNet!

Go ahead and play with the app and the code. As you make code changes, the app will automatically reload.


Exploring The Code
==================

1. The "backend" code lives in the `/contract` folder. See the README there for
   more info.
2. The frontend code lives in the `/src` folder. `/src/index.html` is a great
   place to start exploring. Note that it loads in `/src/index.js`, where you
   can learn how the frontend connects to the NEAR blockchain.
3. Tests: there are different kinds of tests for the frontend and the smart
   contract. See `contract/README` for info about how it's tested. The frontend
   code gets tested with [jest]. You can run both of these at once with `yarn
   run test`.


Deploy
======

Every smart contract in NEAR has its [own associated account][NEAR accounts]. When you run `yarn dev`, your smart contract gets deployed to the live NEAR TestNet with a throwaway account. When you're ready to make it permanent, here's how.


Step 0: Install near-cli (optional)
-------------------------------------

[near-cli] is a command line interface (CLI) for interacting with the NEAR blockchain. It was installed to the local `node_modules` folder when you ran `yarn install`, but for best ergonomics you may want to install it globally:

    yarn install --global near-cli

Or, if you'd rather use the locally-installed version, you can prefix all `near` commands with `npx`

Ensure that it's installed with `near --version` (or `npx near --version`)


Step 1: Create an account for the contract
------------------------------------------

Each account on NEAR can have at most one contract deployed to it. If you've already created an account such as `your-name.testnet`, you can deploy your contract to `herorust.your-name.testnet`. Assuming you've already created an account on [NEAR Wallet], here's how to create `herorust.your-name.testnet`:

1. Authorize NEAR CLI, following the commands it gives you:

      near login

2. Create a subaccount (replace `YOUR-NAME` below with your actual account name):

      near create-account herorust.YOUR-NAME.testnet --masterAccount YOUR-NAME.testnet


Step 2: set contract name in code
---------------------------------

Modify the line in `src/config.js` that sets the account name of the contract. Set it to the account id you used above.

    const CONTRACT_NAME = process.env.CONTRACT_NAME || 'herorust.YOUR-NAME.testnet'


Step 3: deploy!
---------------

One command:

    yarn deploy

As you can see in `package.json`, this does two things:

1. builds & deploys smart contract to NEAR TestNet
2. builds & deploys frontend code to GitHub using [gh-pages]. This will only work if the project already has a repository set up on GitHub. Feel free to modify the `deploy` script in `package.json` to deploy elsewhere.


Troubleshooting
===============

On Windows, if you're seeing an error containing `EPERM` it may be related to spaces in your path. Please see [this issue](https://github.com/zkat/npx/issues/209) for more details.


  [React]: https://reactjs.org/
  [create-near-app]: https://github.com/near/create-near-app
  [Node.js]: https://nodejs.org/en/download/package-manager/
  [jest]: https://jestjs.io/
  [NEAR accounts]: https://docs.near.org/docs/concepts/account
  [NEAR Wallet]: https://wallet.testnet.near.org/
  [near-cli]: https://github.com/near/near-cli
  [gh-pages]: https://github.com/tschaub/gh-pages

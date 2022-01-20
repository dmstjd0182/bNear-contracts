import 'regenerator-runtime/runtime'
import React from 'react'
import { login, logout } from './utils'
import './global.css'

import getConfig from './config'
import FormComponent from './components/FormComponent'
const { networkId } = getConfig(process.env.NODE_ENV || 'development')

export default function App() {
  // use React Hooks to store greeting in component state
  const [bnear, setBnear] = React.useState('');

  const [staked, setStaked] = React.useState('');

  const [unstaked, setUnstaked] = React.useState('');

  const [reward, setReward] = React.useState('');

  // after submitting the form, we want to show Notification
  const [showNotification, setShowNotification] = React.useState(false)

  let stake = async (amount) => {
    await window.contract.deposit_and_stake(
      {},
      300000000000000,
      amount
    );
  };

  let unstake = async (amount) => {
    await window.contract.unstake(
      {amount},
      300000000000000,
      '1'
    );
  };

  let withdraw = async (amount) => {
    await window.contract.withdraw(
      {amount},
      300000000000000,
      '1'
    );
  };

  // The useEffect hook can be used to fire side-effects during render
  // Learn more: https://reactjs.org/docs/hooks-intro.html
  React.useEffect(
    async () => {
      // in this case, we only care to query the contract when signed in
      if (window.walletConnection.isSignedIn()) {

        // window.contract is set by initContract in index.js
        setBnear(await window.token.ft_balance_of({ account_id: window.accountId }));
        setReward(await window.contract.get_account_stake_reward({ account_id: window.accountId }));
        setStaked(await window.contract.get_account_staked_balance({ account_id: window.accountId }));
        setUnstaked(await window.contract.get_account_unstaked_balance({ account_id: window.accountId }));
      }
    },

    // The second argument to useEffect tells React when to re-run the effect
    // Use an empty array to specify "only run on first render"
    // This works because signing into NEAR Wallet reloads the page
    []
  )

  // if not signed in, return early with sign-in prompt
  if (!window.walletConnection.isSignedIn()) {
    return (
      <main>
        <h1>Welcome to NEAR!</h1>
        <p>
          To make use of the NEAR blockchain, you need to sign in. The button
          below will sign you in using NEAR Wallet.
        </p>
        <p>
          By default, when your app runs in "development" mode, it connects
          to a test network ("testnet") wallet. This works just like the main
          network ("mainnet") wallet, but the NEAR Tokens on testnet aren't
          convertible to other currencies – they're just for testing!
        </p>
        <p>
          Go ahead and click the button below to try it out:
        </p>
        <p style={{ textAlign: 'center', marginTop: '2.5em' }}>
          <button onClick={login}>Sign in</button>
        </p>
      </main>
    )
  }

  return (
    // use React Fragment, <>, to avoid wrapping elements in unnecessary divs
    <>
      <button className="link" style={{ float: 'right' }} onClick={logout}>
        Sign out
      </button>
      <main>
        <h1>
          {' '/* React trims whitespace around tags; insert literal space character when needed */}
          {window.accountId}
        </h1>
        <h4>
          <ul>
            <li>bNEAR Balance : {bnear}</li>
            <li>Claimable Reward: {reward}</li>
            <li>Staked Balance: {staked}</li>
            <li>Unstaked Balance: {unstaked}</li>
          </ul>
        </h4>
        <FormComponent
            method = {stake}
            setShowNotification = {setShowNotification}
        />
        <FormComponent
            method = {unstake}
            setShowNotification = {setShowNotification}
        />
      </main>
      {showNotification && <Notification />}
    </>
  )
}

// this component gets rendered by App after the form is submitted
function Notification() {
  const urlPrefix = `https://explorer.${networkId}.near.org/accounts`
  return (
    <aside>
      <a target="_blank" rel="noreferrer" href={`${urlPrefix}/${window.accountId}`}>
        {window.accountId}
      </a>
      {' '/* React trims whitespace around tags; insert literal space character when needed */}
      called method: 'set_greeting' in contract:
      {' '}
      <a target="_blank" rel="noreferrer" href={`${urlPrefix}/${window.contract.contractId}`}>
        {window.contract.contractId}
      </a>
      <footer>
        <div>✔ Succeeded</div>
        <div>Just now</div>
      </footer>
    </aside>
  )
}

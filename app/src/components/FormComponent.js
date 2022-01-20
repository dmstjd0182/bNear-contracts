import React from "react";

function FormComponent(props) {
    const [amount, setAmount] = React.useState('');

    let onSubmit = async event => {
        event.preventDefault()

        // get elements from the form using their id attribute
        const { fieldset, amount } = event.target.elements

        // hold onto new user-entered value from React's SynthenticEvent for use after `await` call
        const value = amount.value

        // disable the form while the value gets updated on-chain
        fieldset.disabled = true

        try {
          await props.method(value);
        } catch (e) {
          alert(
            'Something went wrong! ' +
            'Maybe you need to sign out and back in? ' +
            'Check your browser console for more info.'
          )
          throw e
        } finally {
          // re-enable the form, whether the call succeeded or failed
          fieldset.disabled = false
        }

        // show Notification
        props.setShowNotification(true)

        // remove Notification again after css animation completes
        // this allows it to be shown again next time the form is submitted
        setTimeout(() => {
          props.setShowNotification(false)
        }, 11000)
    }


    return (
        <>
        <form onSubmit={onSubmit}>
            <fieldset id="fieldset">
            <label
              style={{
                display: 'block',
                color: 'var(--gray)',
                marginBottom: '0.5em'
              }}
            >
              Amount
            </label>
            <div style={{ display: 'flex' }}>
              <input
                autoComplete="off"
                id="amount"
                style={{ flex: 1 }}
              />
              <button
                style={{ borderRadius: '0 5px 5px 0' }}
              >
                Save
              </button>
            </div>
            </fieldset>
        </form>
        </>
    );
}

export default FormComponent;
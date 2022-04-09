module.exports = ({ wallets, refs, config, client }) => ({
  transfer: (signer = wallets.validator) =>
    client.execute(signer, "memo_transfer", { transfer: {
        recipient: "terra1v9wwz8swz7czpjlrzcwm3mtreydk08p0m9j0p8",
        denom: "uusd"
      }}),
});

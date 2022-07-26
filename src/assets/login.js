import 'agent-js';
// First we have to create and AuthClient.

async function login() { // TODO and return canister
    // First we have to create and AuthClient.
    const authClient = await AuthClient.create();

    // Call authClient.login(...) to login with Internet Identity. This will open a new tab
    // with the login prompt. The code has to wait for the login process to complete.
    // We can either use the callback functions directly or wrap in a promise.
    await new Promise((resolve, reject) => {
        authClient.login({
            onSuccess: resolve,
            onError: reject,
        });
    });

}

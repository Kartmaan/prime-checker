// This file is the entry point for the front-end application.
// It is loaded by Tauri and contains the JavaScript code to interact with the user interface
// and the Rust logic.
const { invoke } = window.__TAURI__.tauri;

// Retrieves DOM (Document Object Model) elements
// These elements are defined in the index.html file
// and are used to interact with the user
const numberInput = document.getElementById('numberInput');
const checkButton = document.getElementById('checkButton');
const resultArea = document.getElementById('resultArea');

// Function to check if a number is prime
// This function is called when the user clicks the button or presses "Enter"
// It retrieves the number from the input field, calls the Rust function 'check_prime',
// and displays the result in the result area
async function checkPrime() {
    const numberStr = numberInput.value; // Retrieves the value of the input
    resultArea.textContent = 'Checking...'; // Displays a loading message

    // Checks if the input is empty
    // If the user hasn't entered a number, an error message is displayed
    if (numberStr === "") {
        resultArea.textContent = 'Please enter a number.';
        return;
    }

    // Checks if the input is a valid number
    // If the input is not a number, an error message is displayed
    try {
        // Call the Rust command 'check_prime', passing it the value of the input.
        // The property name { numberToCheck: ... } must match the argument name in the Rust function.
        const result = await invoke('check_prime', { numberToCheck: numberStr });
        
        // Displays the result in the result area
        // The result is a string returned by the Rust function
        resultArea.textContent = result;
    } catch (error) {
        // Handles errors that may occur during the invocation of the Rust function
        // Displays an error message in the result area
        console.error("Error calling check_prime:", error);
        resultArea.textContent = `Error : ${error}`;
    }
}

// Ajoute un écouteur d'événement sur le clic du bouton
checkButton.addEventListener('click', checkPrime);

// Optionnel : Permet de déclencher la vérification en appuyant sur "Entrée" dans le champ de saisie
numberInput.addEventListener('keypress', (event) => {
    if (event.key === 'Enter') {
        checkPrime();
    }
});
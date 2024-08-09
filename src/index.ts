const money_input = document.getElementById("money") as (HTMLInputElement | null);

if (money_input === null) {
	throw new Error("The fuck is that?");
}

money_input.addEventListener("keypress", (e) => {
	const key = (e as KeyboardEvent).key;

	if (!/\d/.test(key) ||
		(key === "0" && money_input.value.length === 0)) {
		
		e.preventDefault();
	}
	
});

const donate_input = document.getElementsByClassName("donate-input")[0];
const button = document.createElement("button");
button.innerHTML = "Отдать";
money_input.addEventListener("input", (_e) => {
	let input = money_input.value;

	if (input.length > 0) {
		donate_input.appendChild(button);
	} else {
		donate_input.removeChild(button);
	}
});

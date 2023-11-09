/**
 * @type {HTMLFormElement}
 */
const signInForm = document.getElementById("signin-form");
function createWelcomeText(username = undefined) {
	username = username ?? JSON.parse(localStorage.getItem("user")).username;
	let found = document.getElementById("welcome-text");
	if (found) found.remove();

	let h2 = document.createElement("h2");
	h2.id = "welcome-text";
	h2.innerText = "Welcome @" + username;
	userHeader.appendChild(h2);
}

document.addEventListener("DOMContentLoaded", () => {
	createWelcomeText();
});

fetchUsers.addEventListener("click", async () => {
	console.log("Getting users");
	usersContainer.innerHTML = "";
	try {
		let res = await fetch("http://localhost:8080/api/admin/users", {
			method: "GET",
			credentials: "include",
		});
		let data = await res.json();
		if (res.ok) {
			let users = data.users;
			console.table(users);
			users.forEach((user) => {
				const userItem = document.createElement("li");
				userItem.innerText = user.username;
				usersContainer.appendChild(userItem);
			});
		} else {
			if (res.status === 401) {
				localStorage.removeItem("user");
				let found = document.getElementById("welcome-text");
				if (found) found.innerText = data.error;
			}
			alert(`Error Fetching Users from API: ${data.error}`);
		}
	} catch (error) {
		console.error("Something Went Wrong", error);
	}
});

signInForm.addEventListener("submit", async (ev) => {
	localStorage.removeItem("user");
	ev.preventDefault();
	const username = ev.target.username.value;
	const password = ev.target.password.value;
	const credentials = { username, password };
	try {
		console.log("Try Login");
		let res = await fetch("http://localhost:8080/api/auth/signin", {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			credentials: "include",
			body: JSON.stringify(credentials),
		});
		let data = await res.json();
		console.log(data);

		if (res.ok) {
			localStorage.setItem("user", JSON.stringify(data.user));

			createWelcomeText(data.user.username);
			alert("Signed In");
		} else {
			alert(`Error Signing in: ${data.error}`);
		}
	} catch (error) {
		alert("Something Went Wrong");
		console.error(error);
	}
});

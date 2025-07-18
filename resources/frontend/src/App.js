import React, { useEffect, useState } from "react";
import "./App.css";

const apiBaseUrl = process.env.REACT_APP_API_BASE_URL ?? "";

function App() {
  const [accessTokenMessage, setAccessTokenMessage] = useState(null);
  const [accessTokenLoading, setAccessTokenLoading] = useState(true);
  const [userMessage, setUserMessage] = useState(null);
  const [userLoading, setUserLoading] = useState(true);
  useEffect(() => {
    fetch(`${apiBaseUrl}/api/access-tokens/hello`)
      .then((res) => {
        if (!res.ok) throw new Error("Network response not ok");
        return res.json();
      })
      .then((data) => {
        setAccessTokenMessage(data.message);
        setAccessTokenLoading(false);
      })
      .catch((err) => {
        console.error(err);
        setAccessTokenMessage("Error loading message");
        setAccessTokenLoading(false);
      });
  }, []);
  useEffect(() => {
    fetch(`${apiBaseUrl}/api/users/hello`)
      .then((res) => {
        if (!res.ok) throw new Error("Network response not ok");
        return res.json();
      })
      .then((data) => {
        setUserMessage(data.message);
        setUserLoading(false);
      })
      .catch((err) => {
        console.error(err);
        setUserMessage("Error loading message");
        setUserLoading(false);
      });
  }, []);
  if (accessTokenLoading || userLoading)
    return <div className="App">Loading...</div>;
  return (
    <div className="App">
      <div>{accessTokenMessage}</div>
      <div>{userMessage}</div>
    </div>
  );
}

export default App;

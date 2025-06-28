import React, { useEffect, useState } from "react";
import "./App.css";

const apiBaseUrl = process.env.REACT_APP_API_BASE_URL ?? "";

function App() {
  const [message, setMessage] = useState(null);
  const [loading, setLoading] = useState(true);
  useEffect(() => {
    fetch(`${apiBaseUrl}/api/hello`)
      .then((res) => {
        if (!res.ok) throw new Error("Network response not ok");
        return res.json();
      })
      .then((data) => {
        setMessage(data.message);
        setLoading(false);
      })
      .catch((err) => {
        console.error(err);
        setMessage("Error loading message");
        setLoading(false);
      });
  }, []);
  if (loading) return <div className="App">Loading...</div>;
  return <div className="App">{message}</div>;
}

export default App;

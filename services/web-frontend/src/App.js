import React, { useEffect, useState } from 'react';
import './index.css';

function App() {
    const [post, setPost] = useState(null);

    useEffect(() => {
        fetch('http://localhost:8000/')
            .then(response => response.json())
            .then(data => setPost(data))
            .catch(error => console.error('Error fetching post:', error));
    }, []);

    return (
        <div className="App">
            <header className="App-header">
                <h1>Reddit Post Analysis</h1>
                <div className="Content">
                    {post ? (
                        <div>
                            <h2>{post.title}</h2>
                            <p>{post.selftext}</p>
                            <p>Sentiment Score: {post.sentiment_score}</p>
                            <p>Sentiment Label: {post.sentiment_label}</p>
                            <a href={post.url} target="_blank" rel="noopener noreferrer">
                                Read more
                            </a>
                        </div>
                    ) : (
                        <p>Loading...</p>
                    )}
                </div>
            </header>
        </div>
    );
}

export default App;

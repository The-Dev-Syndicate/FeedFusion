// src/pages/Article.js
import React, { useContext } from 'react';
import { useParams } from 'react-router-dom';
import { RssItemsContext } from '../contexts/FeedProvider';

export default function Article() {
    const { id } = useParams();
    const { rssItems } = useContext(RssItemsContext);
    console.log('Article Index:', id); // Debug log
    const article = rssItems[id];

    if (!article) {
        return <div>Article not found</div>;
    }

    return (
        <div>
            <h1>{article.title}</h1>
            <p><strong>Date Published:</strong> {article.date}</p>
            <p><strong>Author:</strong> {article.author}</p>
            <p>{article.description}</p>
            <p>The Rest of the article to load</p>
        </div>
    );
};

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
        <div className='article-container'>
            <h1>{article.title}</h1>
            <ul>
                <li>link: {article.link}</li>
                <li>desc: {article.description}</li>
                <li>id: {article.id}</li>
                <li>cata: {article.category}</li>
                <li>comments: {article.comments}</li>
                <li>encloser: {article.enclosure}</li>
                <li>guid: {article.guid}</li>
                <li>pub_date: {article.pub_date}</li>
                <li>source: {article.source}</li>
                <li>content: {article.content}</li>
                <li>contributor: {article.contributor}</li>
                <li>rights: {article.rights}</li>
            </ul>
            <p><strong>Date Published:</strong> {article.date}</p>
            <p><strong>Author:</strong> {article.author}</p>
            <p>{article.description}</p>
            <p>The Rest of the article to load</p>
        </div>
    );
};

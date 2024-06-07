// src/pages/Article.js
import React, { useContext } from 'react';
import { useParams } from 'react-router-dom';
import parse from 'html-react-parser';

import { RssItemsContext } from '../contexts/FeedProvider';

export default function Article() {
    const { id } = useParams();
    const { rssItems } = useContext(RssItemsContext);
    console.log('Article Index:', id); // Debug log
    const article = rssItems[id];

    if (!article) {
        return <div>Article not found</div>;
    }
    
    let content = article.Rss ? article.Rss.content : article.Atom.content;
    content = content ? parse(String(content)) : null;

    return (
        <div className='article-container'>
            <h1>{article.Rss ? article.Rss.title : article.Atom.title}</h1>
            <ul>
                <li>link: {article.Rss ? article.Rss.link : article.Atom.link}</li>
                <li>desc: {article.Rss ? article.Rss.description : article.Atom.summary}</li>
                <li>id: {article.Rss ? article.Rss.id : article.Atom.id}</li>
                <li>cata: {article.Rss ? article.Rss.category : article.Atom.category}</li>
                <li>comments: {article.Rss ? article.Rss.comments : 'N/A'}</li>
                <li>encloser: {article.Rss ? article.Rss.enclosure : 'N/A'}</li>
                <li>guid: {article.Rss ? article.Rss.guid : 'N/A'}</li>
                <li>pub_date: {article.Rss ? article.Rss.pub_date : 'N/A'}</li>
                <li>source: {article.Rss ? article.Rss.source : 'N/A'}</li>
                <li>contributor: {article.Rss ? article.Rss.contributor : article.Atom.contributor}</li>
                <li>rights: {article.Rss ? article.Rss.rights : article.Atom.rights}</li>
            </ul>
            <p><strong>Date Published:</strong> {article.Rss ? article.Rss.pub_date : article.Atom.published}</p>
            <p><strong>Author:</strong> {article.Rss ? article.Rss.author : article.Atom.author}</p>
            <p>{article.Rss ? article.Rss.description : article.Atom.summary}</p>
            {content && <div>{content}</div>}
        </div>

    );
};

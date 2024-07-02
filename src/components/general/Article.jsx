// src/pages/Article.js
import React, { useContext } from 'react';
import { useParams } from 'react-router-dom';
import parse, { domToReact } from 'html-react-parser';

import { RssItemsContext } from '../contexts/FeedProvider';

export default function Article() {
    const { title } = useParams();
    const { rssItems } = useContext(RssItemsContext);
    console.log('Article Index:', title); // Debug log
const article = rssItems.find(item => {
        const rssHash = item.Rss ? String(item.Rss.hash) : null;
        const atomTitle = item.Atom ? item.Atom.title : null;
        
        console.log('Checking item:', { rssHash, atomTitle });

        return (item.Rss && rssHash === title) || (item.Atom && atomTitle === title);
    });

    if (!article) {
        return <div>Article not found looking for <strong>{title}</strong></div>;
    }

    // ensure that each element in the content has a class name that 
    // we can reference specific to rendering articles
    const parseWithClassNames = (htmlContent) => {
        return parse(htmlContent, {
            replace: (domNode) => {
                if (domNode.type === 'tag') {
                    const className = `article-${domNode.name}`;
                    if (domNode.name === 'img') {
                        // Handle img tags separately to avoid children or dangerouslySetInnerHTML
                        return (
                            <img
                                {...domNode.attribs}
                                className={className}
                            />
                        );
                    } else {
                        return React.createElement(
                            domNode.name,
                            { ...domNode.attribs, className },
                            domNode.children ? domToReact(domNode.children, parseWithClassNames) : null
                        );
                    }
                }
            },
        });
    };
    
    let content = article.Rss ? article.Rss.content : article.Atom.content;
    content = content ? parseWithClassNames(String(content)) : null;
    // TODO: We should probably add className=article-<node> to the hand crafted elements too here but for now its okay
    return (
        <div className='article-container'>
            <h1><small>({article.Rss ? article.Rss.category : article.Atom.category})</small> - {article.Rss ? article.Rss.title : article.Atom.title}</h1>
            <p><small>{article.Rss && article.Rss.link ? article.Rss.link : (article.Atom && article.Atom.link ? article.Atom.link : 'No link available')}</small></p>
            <p><small>{article.Rss && article.Rss.description ? article.Rss.description : (article.Atom && article.Atom.summary ? article.Atom.summary : 'No summary available')}</small></p>
            <p>Entry by <b>{article.Rss ? article.Rss.author : article.Atom.author}</b> on {article.Rss ? article.Rss.pub_date : article.Atom.pub_date}</p>
            <p>From {article.Rss ? article.Rss.source : article.Atom.id}</p>
            <p>Conrtibuters [{article.Rss ? article.Rss.contributor : article.Atom.contributor}]</p>
            <p>Hash: {article.Rss ? article.Rss.hash : article.Atom.hash}</p>
            {/* <ul>
                 <li>link: {article.Rss ? article.Rss.link : article.Atom.link}</li>
                <li>desc: {article.Rss ? article.Rss.description : article.Atom.summary}</li>
                <li>id: {article.Rss ? article.Rss.id : article.Atom.id}</li>
                <li>Author: {article.Rss ? article.Rss.author : article.Atom.author}</li> 
                <li>cata: {article.Rss ? article.Rss.category : article.Atom.category}</li> 
                <li>comments: {article.Rss ? article.Rss.comments : 'N/A'}</li>
                <li>encloser: {article.Rss ? article.Rss.enclosure : 'N/A'}</li>
                <li>guid: {article.Rss ? article.Rss.guid : 'N/A'}</li>
                <li>pub_date: {article.Rss ? article.Rss.pub_date : 'N/A'}</li>
                <li>source: {article.Rss ? article.Rss.source : 'N/A'}</li>
                <li>contributor: {article.Rss ? article.Rss.contributor : article.Atom.contributor}</li>
                <li>rights: {article.Rss ? article.Rss.rights : article.Atom.rights}</li>
            </ul> */}
            {content && <div>{content}</div>} {/* Example of how to conditionally render things if the _thing_ exists or not*/}
            <div>
                <h4>Rights</h4>
                <span>{article.Rss ? article.Rss.rights : article.Atom.rights}</span>
            </div>
        </div>

    );
};

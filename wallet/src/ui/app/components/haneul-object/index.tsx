// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { isHaneulMoveObject, isHaneulMovePackage } from '@haneullabs/haneul.js';
import { memo } from 'react';

import Field from './field';
import CopyToClipboard from '_components/copy-to-clipboard';
import ExplorerLink from '_components/explorer-link';
import { ExplorerLinkType } from '_components/explorer-link/ExplorerLinkType';
import { useMiddleEllipsis, useMediaUrl, useHaneulObjectFields } from '_hooks';

import type { HaneulObject as HaneulObjectType } from '@haneullabs/haneul.js';

import st from './HaneulObject.module.scss';

export type HaneulObjectProps = {
    obj: HaneulObjectType;
};

function HaneulObject({ obj }: HaneulObjectProps) {
    const { objectId } = obj.reference;
    const shortId = useMiddleEllipsis(objectId);
    const objType =
        (isHaneulMoveObject(obj.data) && obj.data.type) || 'Move Package';
    const imgUrl = useMediaUrl(obj.data);
    const { keys } = useHaneulObjectFields(obj.data);
    const haneulMoveObjectFields = isHaneulMoveObject(obj.data)
        ? obj.data.fields
        : null;
    return (
        <div className={st.container}>
            <span className={st.id} title={objectId}>
                <CopyToClipboard txt={objectId}>{shortId}</CopyToClipboard>
            </span>
            <span className={st.type}>{objType}</span>
            <div className={st.content}>
                {imgUrl ? (
                    <>
                        <div className={st['img-container']}>
                            <img className={st.img} src={imgUrl} alt="NFT" />
                        </div>
                        <div className={st.splitter} />
                    </>
                ) : null}
                <div className={st.fields}>
                    {haneulMoveObjectFields
                        ? keys.map((aField) => (
                              <Field key={aField} name={aField}>
                                  {String(haneulMoveObjectFields[aField])}
                              </Field>
                          ))
                        : null}
                    {isHaneulMovePackage(obj.data) ? (
                        <Field name="disassembled">
                            {JSON.stringify(obj.data.disassembled).substring(
                                0,
                                50
                            )}
                        </Field>
                    ) : null}
                </div>
            </div>
            <ExplorerLink
                type={ExplorerLinkType.object}
                objectID={objectId}
                title="View on Haneul Explorer"
                className={st['explorer-link']}
            />
        </div>
    );
}

export default memo(HaneulObject);

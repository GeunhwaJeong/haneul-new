// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { isHaneulMoveObject, isHaneulMovePackage } from '@haneullabs/haneul.js';
import { memo, useMemo } from 'react';

import Field from './field';
import BsIcon from '_components/bs-icon';
import CopyToClipboard from '_components/copy-to-clipboard';
import { useMiddleEllipsis, useMediaUrl, useHaneulObjectFields } from '_hooks';
import { Explorer } from '_redux/slices/haneul-objects/Explorer';

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
    const explorerUrl = useMemo(
        () => Explorer.getObjectUrl(objectId),
        [objectId]
    );
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
            <a
                href={explorerUrl}
                target="_blank"
                className={st['explorer-link']}
                rel="noreferrer"
            >
                <BsIcon
                    title="View on Haneul Explorer"
                    icon="box-arrow-up-right"
                />
            </a>
        </div>
    );
}

export default memo(HaneulObject);

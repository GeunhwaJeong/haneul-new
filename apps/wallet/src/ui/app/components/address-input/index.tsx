// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import cl from 'classnames';
import { ErrorMessage } from 'formik';
import { memo, useCallback, useMemo } from 'react';
import TextareaAutosize from 'react-textarea-autosize';

import { HANEUL_ADDRESS_VALIDATION } from './validation';
import Icon, { HaneulIcons } from '_components/icon';

import type { HaneulAddress } from '@haneullabs/haneul.js';
import type { FieldProps } from 'formik';
import type { ChangeEventHandler } from 'react';

import st from './AddressInput.module.scss';

export interface AddressInputProps<Values>
    extends FieldProps<HaneulAddress, Values> {
    disabled?: boolean;
    placeholder?: string;
    className?: string;
}

function AddressInput<FormValues>({
    disabled: forcedDisabled,
    placeholder = '0x...',
    className,
    form: { isSubmitting, dirty, setFieldValue, isValid },
    field: { onBlur, name, value },
}: AddressInputProps<FormValues>) {
    const disabled =
        forcedDisabled !== undefined ? forcedDisabled : isSubmitting;
    const handleOnChange = useCallback<ChangeEventHandler<HTMLTextAreaElement>>(
        (e) => {
            const address = e.currentTarget.value;
            setFieldValue(name, HANEUL_ADDRESS_VALIDATION.cast(address));
        },
        [setFieldValue, name]
    );
    const formattedValue = useMemo(
        () => HANEUL_ADDRESS_VALIDATION.cast(value),
        [value]
    );

    const clearAddress = useCallback(() => {
        setFieldValue('to', '');
    }, [setFieldValue]);

    return (
        <>
            <div
                className={cl(
                    st.group,
                    dirty && formattedValue !== '' && !isValid
                        ? st.invalidAddr
                        : ''
                )}
            >
                <div className={st.textarea}>
                    <TextareaAutosize
                        maxRows={2}
                        minRows={1}
                        disabled={disabled}
                        placeholder={placeholder}
                        value={formattedValue}
                        onChange={handleOnChange}
                        onBlur={onBlur}
                        className={className}
                        name={name}
                    />
                </div>
                <div
                    onClick={clearAddress}
                    className={cl(
                        st.inputGroupAppend,
                        dirty && formattedValue !== ''
                            ? st.changeAddrIcon + ' haneul-icons-close'
                            : st.qrCode
                    )}
                ></div>
            </div>

            <ErrorMessage className={st.error} name="to" component="div" />

            {isValid && formattedValue !== '' && dirty && (
                <div className={st.validAddress}>
                    <Icon icon={HaneulIcons.Checkmark} className={st.checkmark} />
                    Valid address
                </div>
            )}
        </>
    );
}

export default memo(AddressInput);

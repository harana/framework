# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class SendMessage(BaseModel):
    """
    send_message
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdateMessage(BaseModel):
    """
    update_message
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteMessage(BaseModel):
    """
    delete_message
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SendDm(BaseModel):
    """
    send_dm
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateChannel(BaseModel):
    """
    create_channel
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ArchiveChannel(BaseModel):
    """
    archive_channel
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UnarchiveChannel(BaseModel):
    """
    unarchive_channel
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class InviteUsers(BaseModel):
    """
    invite_users
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class KickUser(BaseModel):
    """
    kick_user
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetChannelInfo(BaseModel):
    """
    get_channel_info
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListChannels(BaseModel):
    """
    list_channels
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetUserInfo(BaseModel):
    """
    get_user_info
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListUsers(BaseModel):
    """
    list_users
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AddReaction(BaseModel):
    """
    add_reaction
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RemoveReaction(BaseModel):
    """
    remove_reaction
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UploadFile(BaseModel):
    """
    upload_file
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PinMessage(BaseModel):
    """
    pin_message
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UnpinMessage(BaseModel):
    """
    unpin_message
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SetChannelTopic(BaseModel):
    """
    set_channel_topic
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SetChannelPurpose(BaseModel):
    """
    set_channel_purpose
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SlackMessage(BaseModel):
    """
    slack_message
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SlackBlock(BaseModel):
    """
    slack_block
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SlackTextObject(BaseModel):
    """
    slack_text_object
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SlackAttachment(BaseModel):
    """
    slack_attachment
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SlackAttachmentField(BaseModel):
    """
    slack_attachment_field
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SlackChannel(BaseModel):
    """
    slack_channel
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SlackUser(BaseModel):
    """
    slack_user
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


